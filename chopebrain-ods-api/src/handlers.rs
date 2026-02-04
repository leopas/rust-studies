//! Rotas e handlers: login, hambúrgueres vendidos, hambúrgueres comandas antigas.

use crate::auth::{self, create_token, validate_login, LoginRequest, LoginResponse};
use crate::config::Config;
use axum::{
    middleware,
    routing::post,
    Router,
    extract::State,
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool};
use std::error::Error;
use std::sync::Arc;

/// Loga erro completo: mensagem, cadeia de causas e backtrace (em debug).
fn log_error_full(context: &str, e: &dyn Error) {
    tracing::error!("{}: {}", context, e);
    let mut source = e.source();
    while let Some(s) = source {
        tracing::error!("  caused by: {}", s);
        source = s.source();
    }
    #[cfg(debug_assertions)]
    {
        tracing::error!("  backtrace:\n{:?}", std::backtrace::Backtrace::capture());
    }
}

pub type AppState = (Arc<Config>, Arc<MySqlPool>);

pub fn router(config: Arc<Config>, pool: Arc<MySqlPool>) -> Router {
    let state: AppState = (config, pool);
    Router::new()
        .route("/api/auth/login", post(login))
        .route(
            "/api/ods/hamburgueres-vendidos",
            post(hamburgueres_vendidos)
                .route_layer(middleware::from_fn_with_state(state.clone(), auth::require_jwt)),
        )
        .route(
            "/api/ods/hamburgueres-comandas-antigas",
            post(hamburgueres_comandas_antigas)
                .route_layer(middleware::from_fn_with_state(state.clone(), auth::require_jwt)),
        )
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .with_state(state)
}

async fn login(
    State((config, _)): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!("handler: POST /api/auth/login");
    if !validate_login(config.as_ref(), &req) {
        tracing::warn!("handler: POST /api/auth/login -> 401 credenciais inválidas");
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({ "error": "Credenciais inválidas" })),
        ));
    }
    let sub = req
        .username
        .as_deref()
        .unwrap_or("api")
        .to_string();
    let token = create_token(config.as_ref(), &sub).map_err(|e| {
        log_error_full("handler: POST /api/auth/login -> 500 erro ao criar JWT", e.as_ref());
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
    })?;
    tracing::info!("handler: POST /api/auth/login -> 200 OK (token gerado para sub={})", sub);
    Ok(Json(LoginResponse { token }))
}

#[derive(Debug, Deserialize)]
pub struct MesRequest {
    pub mes: String, // "YYYY-MM"
}

// Regra hambúrguer: categoria etiquetas.descricao IN ('Hamburger','Hamburguer') OU nome do produto com palavras-chave.
const HAMBURGER_CATEGORY_FILTER: &str = "AND (LOWER(e.descricao) IN ('hamburger','hamburguer') OR LOWER(pr.nome) LIKE '%hamburguer%' OR LOWER(pr.nome) LIKE '%burger%' OR LOWER(pr.nome) LIKE '%cheeseburger%' OR LOWER(pr.nome) LIKE '%bacon burger%' OR LOWER(pr.nome) LIKE '%frango burger%' OR LOWER(pr.nome) LIKE '%artesanal burger%')";

#[derive(Debug, Serialize)]
pub struct HamburgueresVendidosResponse {
    pub totais: Totais,
    pub por_categoria: Vec<PorCategoria>,
    pub por_produto: Vec<PorProduto>,
    pub itens: Vec<ItemVenda>,
}

#[derive(Debug, Serialize)]
pub struct Totais {
    pub quantidade: i64,
    pub valor: f64,
    pub ticket_medio: f64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct PorCategoria {
    pub categoria: Option<String>,
    pub quantidade: i64,
    pub valor: f64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct PorProduto {
    pub produto: String,
    pub categoria: Option<String>,
    pub quantidade: i64,
    pub valor: f64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct ItemVenda {
    pub data_venda_item: Option<String>,
    pub pedido: i64,
    pub produto: String,
    pub categoria: Option<String>,
    pub quantidade: i64,
    pub valor: f64,
}

async fn hamburgueres_vendidos(
    State((_config, pool)): State<AppState>,
    Json(body): Json<MesRequest>,
) -> Result<Json<HamburgueresVendidosResponse>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!("handler: POST /api/ods/hamburgueres-vendidos mes={}", body.mes);
    let (ano, mes) = parse_mes(&body.mes)?;
    tracing::debug!("handler: hamburgueres-vendidos ano={} mes={}", ano, mes);
    let total: Option<(i64, f64)> = sqlx::query_as(
        &format!(
            r#"
            SELECT CAST(COALESCE(SUM(ip.quantidade), 0) AS SIGNED), COALESCE(SUM(ip.quantidade * ip.valorunitario), 0)
            FROM itenspedido ip
            INNER JOIN pedidos ped ON ped.codigo = ip.codigopedido
            INNER JOIN produtodetalhe pd ON pd.codigo = ip.codigoprodutodetalhe
            INNER JOIN produtos pr ON pr.codigo = ip.codigoproduto
            LEFT JOIN etiquetas e ON e.codigo = pr.codigoetiqueta
            WHERE YEAR(ip.datahoracadastro) = ? AND MONTH(ip.datahoracadastro) = ?
            {}
            "#,
            HAMBURGER_CATEGORY_FILTER
        ),
    )
    .bind(ano)
    .bind(mes)
    .fetch_optional(pool.as_ref())
    .await
    .map_err(|e| {
        log_error_full("handler: hamburgueres-vendidos erro MySQL (totais)", &e as &dyn Error);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() })))
    })?;

    let (quantidade, valor) = total.unwrap_or((0, 0.0));
    let ticket_medio = if quantidade > 0 { valor / (quantidade as f64) } else { 0.0 };
    tracing::debug!("handler: hamburgueres-vendidos totais quantidade={} valor={}", quantidade, valor);

    let por_categoria: Vec<PorCategoria> = sqlx::query_as(&format!(
        r#"
        SELECT e.descricao AS categoria, CAST(COALESCE(SUM(ip.quantidade), 0) AS SIGNED) AS quantidade, COALESCE(SUM(ip.quantidade * ip.valorunitario), 0) AS valor
        FROM itenspedido ip
        INNER JOIN pedidos ped ON ped.id = ip.pedidoid
        INNER JOIN produtodetalhe pd ON pd.id = ip.produtodetalheid
        INNER JOIN produtos pr ON pr.id = pd.produtoid
        LEFT JOIN etiquetas e ON e.codigo = pr.codigoetiqueta
        WHERE YEAR(ip.datahoracadastro) = ? AND MONTH(ip.datahoracadastro) = ?
        {}
        GROUP BY e.descricao
        "#,
        HAMBURGER_CATEGORY_FILTER
    ))
    .bind(ano)
    .bind(mes)
    .fetch_all(pool.as_ref())
    .await
    .map_err(|e| {
        log_error_full("handler: hamburgueres-vendidos erro MySQL (por_categoria)", &e as &dyn Error);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() })))
    })?;

    let por_produto: Vec<PorProduto> = sqlx::query_as(&format!(
        r#"
        SELECT pr.nome AS produto, e.descricao AS categoria, CAST(COALESCE(SUM(ip.quantidade), 0) AS SIGNED) AS quantidade, COALESCE(SUM(ip.quantidade * ip.valorunitario), 0) AS valor
        FROM itenspedido ip
        INNER JOIN pedidos ped ON ped.codigo = ip.codigopedido
        INNER JOIN produtodetalhe pd ON pd.codigo = ip.codigoprodutodetalhe
        INNER JOIN produtos pr ON pr.codigo = ip.codigoproduto
        LEFT JOIN etiquetas e ON e.codigo = pr.codigoetiqueta
        WHERE YEAR(ip.datahoracadastro) = ? AND MONTH(ip.datahoracadastro) = ?
        {}
        GROUP BY pr.codigo, pr.nome, e.descricao
        ORDER BY quantidade DESC
        LIMIT 10
        "#,
        HAMBURGER_CATEGORY_FILTER
    ))
    .bind(ano)
    .bind(mes)
    .fetch_all(pool.as_ref())
    .await
    .map_err(|e| {
        log_error_full("handler: hamburgueres-vendidos erro MySQL (por_produto)", &e as &dyn Error);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() })))
    })?;

    let itens: Vec<ItemVenda> = sqlx::query_as(&format!(
        r#"
        SELECT DATE_FORMAT(ip.datahoracadastro, '%Y-%m-%d %H:%i') AS data_venda_item, ped.codigo AS pedido, pr.nome AS produto, e.descricao AS categoria, CAST(ip.quantidade AS SIGNED) AS quantidade, (ip.quantidade * ip.valorunitario) AS valor
        FROM itenspedido ip
        INNER JOIN pedidos ped ON ped.id = ip.pedidoid
        INNER JOIN produtodetalhe pd ON pd.id = ip.produtodetalheid
        INNER JOIN produtos pr ON pr.id = pd.produtoid
        LEFT JOIN etiquetas e ON e.codigo = pr.codigoetiqueta
        WHERE YEAR(ip.datahoracadastro) = ? AND MONTH(ip.datahoracadastro) = ?
        {}
        ORDER BY ip.datahoracadastro, ped.codigo
        "#,
        HAMBURGER_CATEGORY_FILTER
    ))
    .bind(ano)
    .bind(mes)
    .fetch_all(pool.as_ref())
    .await
    .map_err(|e| {
        log_error_full("handler: hamburgueres-vendidos erro MySQL (itens)", &e as &dyn Error);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() })))
    })?;

    tracing::info!("handler: POST /api/ods/hamburgueres-vendidos -> 200 OK (quantidade={}, valor={})", quantidade, valor);
    Ok(Json(HamburgueresVendidosResponse {
        totais: Totais {
            quantidade,
            valor,
            ticket_medio,
        },
        por_categoria,
        por_produto,
        itens,
    }))
}

fn parse_mes(mes: &str) -> Result<(i32, u32), (StatusCode, Json<serde_json::Value>)> {
    let parts: Vec<&str> = mes.split('-').collect();
    if parts.len() != 2 {
        tracing::warn!("handler: parse_mes inválido (esperado YYYY-MM): {:?}", mes);
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "mes deve ser YYYY-MM" })),
        ));
    }
    let ano: i32 = parts[0].parse().map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "ano inválido" })),
        )
    })?;
    let mes: u32 = parts[1].parse().map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "mês inválido" })),
        )
    })?;
    if mes < 1 || mes > 12 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "mês deve ser 01-12" })),
        ));
    }
    Ok((ano, mes))
}

// --- Hamburgueres comandas antigas ---

#[derive(Debug, Serialize)]
pub struct HamburgueresComandasAntigasResponse {
    pub totais: Totais,
    pub por_produto: Vec<PorProduto>,
    pub por_pedido: Vec<PorPedido>,
    pub detalhamento: Vec<DetalheComandaAntiga>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct PorPedido {
    pub pedido_id: i64,
    pub data_abertura: Option<String>,
    pub data_fechamento: Option<String>,
    pub itens: i64,
    pub quantidade: i64,
    pub valor: f64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct DetalheComandaAntiga {
    pub data_venda_item: Option<String>,
    pub pedido: i64,
    pub data_abertura: Option<String>,
    pub dias_diferenca: Option<i64>,
    pub produto: String,
    pub quantidade: i64,
    pub valor: f64,
}

async fn hamburgueres_comandas_antigas(
    State((_config, pool)): State<AppState>,
    Json(body): Json<MesRequest>,
) -> Result<Json<HamburgueresComandasAntigasResponse>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!("handler: POST /api/ods/hamburgueres-comandas-antigas mes={}", body.mes);
    let (ano, mes) = parse_mes(&body.mes)?;
    tracing::debug!("handler: hamburgueres-comandas-antigas ano={} mes={}", ano, mes);
    let comanda_antiga_filter = format!(
        "AND (YEAR(p.dataabertura) < {} OR (YEAR(p.dataabertura) = {} AND MONTH(p.dataabertura) < {}))",
        ano, ano, mes
    );
    let base_filter = format!(
        "WHERE YEAR(ip.datahoracadastro) = {} AND MONTH(ip.datahoracadastro) = {} {} ",
        ano, mes, comanda_antiga_filter
    );

    let total_quantidade: i64 = sqlx::query_scalar(&format!(
        r#"
        SELECT CAST(COALESCE(SUM(ip.quantidade), 0) AS SIGNED)
        FROM itenspedido ip
        INNER JOIN pedidos p ON p.codigo = ip.codigopedido
        INNER JOIN produtodetalhe pd ON pd.codigo = ip.codigoprodutodetalhe
        INNER JOIN produtos pr ON pr.codigo = ip.codigoproduto
        LEFT JOIN etiquetas e ON e.codigo = pr.codigoetiqueta
        {} {}
        "#,
        base_filter, HAMBURGER_CATEGORY_FILTER
    ))
    .fetch_one(pool.as_ref())
    .await
    .map_err(|e| {
        log_error_full("handler: hamburgueres-comandas-antigas erro MySQL (total_quantidade)", &e as &dyn Error);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() })))
    })?;

    let total_valor: f64 = sqlx::query_scalar(&format!(
        r#"
        SELECT COALESCE(SUM(ip.quantidade * ip.valorunitario), 0)
        FROM itenspedido ip
        INNER JOIN pedidos p ON p.codigo = ip.codigopedido
        INNER JOIN produtodetalhe pd ON pd.codigo = ip.codigoprodutodetalhe
        INNER JOIN produtos pr ON pr.codigo = ip.codigoproduto
        LEFT JOIN etiquetas e ON e.codigo = pr.codigoetiqueta
        {} {}
        "#,
        base_filter, HAMBURGER_CATEGORY_FILTER
    ))
    .fetch_one(pool.as_ref())
    .await
    .map_err(|e| {
        log_error_full("handler: hamburgueres-comandas-antigas erro MySQL (total_valor)", &e as &dyn Error);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() })))
    })?;

    let quantidade = total_quantidade;
    let valor = total_valor;
    let ticket_medio = if quantidade > 0 { valor / (quantidade as f64) } else { 0.0 };

    let por_produto: Vec<PorProduto> = sqlx::query_as(&format!(
        r#"
        SELECT pr.nome AS produto, e.descricao AS categoria, CAST(COALESCE(SUM(ip.quantidade), 0) AS SIGNED) AS quantidade, COALESCE(SUM(ip.quantidade * ip.valorunitario), 0) AS valor
        FROM itenspedido ip
        INNER JOIN pedidos p ON p.codigo = ip.codigopedido
        INNER JOIN produtodetalhe pd ON pd.codigo = ip.codigoprodutodetalhe
        INNER JOIN produtos pr ON pr.codigo = ip.codigoproduto
        LEFT JOIN etiquetas e ON e.codigo = pr.codigoetiqueta
        {} {}
        GROUP BY pr.codigo, pr.nome, e.descricao
        ORDER BY quantidade DESC
        "#,
        base_filter, HAMBURGER_CATEGORY_FILTER
    ))
    .fetch_all(pool.as_ref())
    .await
    .map_err(|e| {
        log_error_full("handler: hamburgueres-comandas-antigas erro MySQL (por_produto)", &e as &dyn Error);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() })))
    })?;

    let por_pedido: Vec<PorPedido> = sqlx::query_as(&format!(
        r#"
        SELECT p.codigo AS pedido_id, DATE_FORMAT(p.dataabertura, '%Y-%m-%d %H:%i') AS data_abertura, DATE_FORMAT(p.datafechamento, '%Y-%m-%d %H:%i') AS data_fechamento,
               COUNT(*) AS itens, CAST(COALESCE(SUM(ip.quantidade), 0) AS SIGNED) AS quantidade, COALESCE(SUM(ip.quantidade * ip.valorunitario), 0) AS valor
        FROM itenspedido ip
        INNER JOIN pedidos p ON p.codigo = ip.codigopedido
        INNER JOIN produtodetalhe pd ON pd.codigo = ip.codigoprodutodetalhe
        INNER JOIN produtos pr ON pr.codigo = ip.codigoproduto
        LEFT JOIN etiquetas e ON e.codigo = pr.codigoetiqueta
        {} {}
        GROUP BY p.codigo, p.dataabertura, p.datafechamento
        ORDER BY p.dataabertura
        "#,
        base_filter, HAMBURGER_CATEGORY_FILTER
    ))
    .fetch_all(pool.as_ref())
    .await
    .map_err(|e| {
        log_error_full("handler: hamburgueres-comandas-antigas erro MySQL (por_pedido)", &e as &dyn Error);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() })))
    })?;

    let detalhamento: Vec<DetalheComandaAntiga> = sqlx::query_as(&format!(
        r#"
        SELECT DATE_FORMAT(ip.datahoracadastro, '%Y-%m-%d %H:%i') AS data_venda_item, p.codigo AS pedido, DATE_FORMAT(p.dataabertura, '%Y-%m-%d') AS data_abertura,
               DATEDIFF(ip.datahoracadastro, p.dataabertura) AS dias_diferenca, pr.nome AS produto, CAST(ip.quantidade AS SIGNED) AS quantidade, (ip.quantidade * ip.valorunitario) AS valor
        FROM itenspedido ip
        INNER JOIN pedidos p ON p.codigo = ip.codigopedido
        INNER JOIN produtodetalhe pd ON pd.codigo = ip.codigoprodutodetalhe
        INNER JOIN produtos pr ON pr.codigo = ip.codigoproduto
        LEFT JOIN etiquetas e ON e.codigo = pr.codigoetiqueta
        {} {}
        ORDER BY ip.datahoracadastro, p.codigo
        "#,
        base_filter, HAMBURGER_CATEGORY_FILTER
    ))
    .fetch_all(pool.as_ref())
    .await
    .map_err(|e| {
        log_error_full("handler: hamburgueres-comandas-antigas erro MySQL (detalhamento)", &e as &dyn Error);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() })))
    })?;

    tracing::info!("handler: POST /api/ods/hamburgueres-comandas-antigas -> 200 OK (quantidade={}, valor={})", quantidade, valor);
    Ok(Json(HamburgueresComandasAntigasResponse {
        totais: Totais {
            quantidade,
            valor,
            ticket_medio,
        },
        por_produto,
        por_pedido,
        detalhamento,
    }))
}
