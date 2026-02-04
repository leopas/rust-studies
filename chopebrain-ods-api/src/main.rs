use chopebrain_ods_api::{config, db, handlers, mtls};
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "chopebrain_ods_api=debug,tower_http=debug,info".into()),
        ))
        .with(tracing_subscriber::fmt::layer().with_target(true).with_thread_ids(false))
        .init();

    tracing::info!("=== Iniciando Chopebrain ODS API ===");

    let work_dir = config::load_dotenv_from_root()?;
    tracing::info!("config: .env carregado da raiz: {}", work_dir.display());

    let cfg = config::Config::from_env().map_err(|e| {
        tracing::error!("config: falha ao carregar: {}", e);
        anyhow::anyhow!("{}", e)
    })?;
    let cfg = Arc::new(cfg);
    tracing::debug!("config: ODS_HOST={}, ODS_NAME={}, JWT_EXPIRATION_DAYS={}", cfg.ods_host, cfg.ods_name, cfg.jwt_expiration_days);

    tracing::info!("db: criando pool MySQL ODS...");
    let pool = db::create_pool(&cfg).await.map_err(|e| {
        tracing::error!("db: falha ao conectar: {}", e);
        anyhow::anyhow!("{}", e)
    })?;
    let pool = Arc::new(pool);
    tracing::info!("db: pool MySQL ODS conectado com sucesso");

    let app = handlers::router(cfg.clone(), pool);

    let listen = std::env::var("LISTEN").unwrap_or_else(|_| "0.0.0.0:3000".into());
    let addr: std::net::SocketAddr = listen.parse()?;

    if mtls::mtls_enabled(&cfg) {
        tracing::info!("server: iniciando HTTPS com mTLS em https://{}", addr);
        mtls::serve_mtls(app, addr, &cfg).await?;
    } else {
        tracing::info!("server: iniciando HTTP em http://{}", addr);
        let listener = tokio::net::TcpListener::bind(addr).await?;
        tracing::info!("server: escutando em {}", addr);
        axum::serve(listener, app).await?;
    }
    Ok(())
}
