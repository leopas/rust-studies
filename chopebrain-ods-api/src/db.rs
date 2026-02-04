//! Pool MySQL ODS com SSL (ODS_SSL_CA).

use crate::config::Config;
use sqlx::mysql::{MySqlConnectOptions, MySqlSslMode};
use sqlx::MySqlPool;

pub async fn create_pool(cfg: &Config) -> anyhow::Result<MySqlPool> {
    tracing::debug!("db: host={}:{}, database={}, user={}", cfg.ods_host, cfg.ods_port, cfg.ods_name, cfg.ods_user);

    let mut opts = MySqlConnectOptions::new()
        .host(&cfg.ods_host)
        .port(cfg.ods_port)
        .username(&cfg.ods_user)
        .password(&cfg.ods_password)
        .database(&cfg.ods_name);

    if let Some(ca_path) = cfg.ods_ssl_ca_path() {
        if ca_path.exists() {
            tracing::info!("db: SSL CA: {:?}", ca_path);
            opts = opts
                .ssl_mode(MySqlSslMode::VerifyCa)
                .ssl_ca(ca_path);
        } else {
            tracing::warn!("db: ODS_SSL_CA não encontrado: {:?}, usando SSL padrão", ca_path);
        }
    } else {
        tracing::debug!("db: ODS_SSL_CA não configurado");
    }

    let pool = MySqlPool::connect_with(opts).await.map_err(|e| {
        tracing::error!("db: erro ao conectar MySQL: {}", e);
        e
    })?;
    tracing::debug!("db: pool criado (size config padrão)");
    Ok(pool)
}
