//! Autenticação JWT: login e validação de token.

use crate::config::Config;
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: Option<String>,
    pub password: Option<String>,
    #[serde(rename = "secret")]
    pub secret: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
}

/// Valida credenciais: AUTH_USERNAME/AUTH_PASSWORD ou AUTH_SECRET (único segredo).
pub fn validate_login(cfg: &Config, req: &LoginRequest) -> bool {
    tracing::debug!("auth: tentativa de login (username presente={}, password presente={}, secret presente={})",
        req.username.is_some(), req.password.is_some(), req.secret.is_some());
    if let Some(secret) = &cfg.auth_secret {
        if req.secret.as_deref() == Some(secret.as_str()) {
            tracing::info!("auth: login OK via AUTH_SECRET (secret)");
            return true;
        }
        if req.password.as_deref() == Some(secret.as_str()) {
            tracing::info!("auth: login OK via AUTH_SECRET (password)");
            return true;
        }
    }
    if let (Some(u), Some(p)) = (&cfg.auth_username, &cfg.auth_password) {
        if req.username.as_deref() == Some(u.as_str()) && req.password.as_deref() == Some(p.as_str()) {
            tracing::info!("auth: login OK via AUTH_USERNAME/AUTH_PASSWORD (user={})", u);
            return true;
        }
    }
    tracing::warn!("auth: login recusado (credenciais inválidas)");
    false
}

pub fn create_token(cfg: &Config, sub: &str) -> anyhow::Result<String> {
    let now = Utc::now();
    let exp = now + Duration::days(cfg.jwt_expiration_days as i64);
    let claims = Claims {
        sub: sub.to_string(),
        exp: exp.timestamp(),
        iat: now.timestamp(),
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(cfg.jwt_secret.as_bytes()),
    )?;
    tracing::info!("auth: JWT criado para sub={}, expira em {} dias", sub, cfg.jwt_expiration_days);
    Ok(token)
}

pub fn decode_token(cfg: &Config, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(cfg.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map(|d| d.claims)
}

/// Middleware: exige Authorization: Bearer <token> e valida JWT.
pub async fn require_jwt(
    State((config, _)): State<(Arc<Config>, Arc<sqlx::MySqlPool>)>,
    request: Request,
    next: Next,
) -> Result<Response, impl IntoResponse> {
    let path = request.uri().path().to_string();
    let auth = request
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "));
    let token = match auth {
        Some(t) => t,
        None => {
            tracing::warn!("auth: {} - 401 Authorization header missing or invalid", path);
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({ "error": "Authorization header missing or invalid" })),
            ));
        }
    };
    match decode_token(config.as_ref(), token) {
        Ok(claims) => {
            tracing::debug!("auth: {} - JWT válido sub={}", path, claims.sub);
            Ok(next.run(request).await)
        }
        Err(e) => {
            tracing::warn!("auth: {} - 401 token inválido ou expirado: {}", path, e);
            Err((
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({ "error": "Invalid or expired token" })),
            ))
        }
    }
}
