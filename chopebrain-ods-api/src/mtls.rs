//! Servidor HTTPS com mTLS (exige certificado do cliente).

use crate::config::Config;
use axum::Router;
use axum_server::tls_rustls::RustlsConfig;
use rustls::server::WebPkiClientVerifier;
use rustls_pemfile::{certs, private_key};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::Arc;

pub fn mtls_enabled(cfg: &Config) -> bool {
    if std::env::var("FORCE_HTTP").as_deref() == Ok("1") {
        return false;
    }
    let server_cert = cfg
        .mtls_server_cert
        .as_ref()
        .map(|p| cfg.resolve_path(p))
        .or_else(|| Some(cfg.work_dir.join("certs-mtls").join("server-cert.pem")));
    let server_key = cfg
        .mtls_server_key
        .as_ref()
        .map(|p| cfg.resolve_path(p))
        .or_else(|| Some(cfg.work_dir.join("certs-mtls").join("server-key.pem")));
    server_cert.map(|p| p.exists()).unwrap_or(false)
        && server_key.map(|p| p.exists()).unwrap_or(false)
}

fn load_cert(path: &Path) -> anyhow::Result<Vec<rustls::pki_types::CertificateDer<'static>>> {
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);
    certs(&mut reader).collect::<Result<Vec<_>, _>>().map_err(|e| anyhow::anyhow!("{:?}", e))
}

fn load_private_key(
    path: &Path,
) -> anyhow::Result<rustls::pki_types::PrivateKeyDer<'static>> {
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);
    private_key(&mut reader)?
        .ok_or_else(|| anyhow::anyhow!("Nenhuma chave privada encontrada em {:?}", path))
}

fn load_ca_certs(
    path: &Path,
) -> anyhow::Result<Vec<rustls::pki_types::CertificateDer<'static>>> {
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);
    certs(&mut reader).collect::<Result<Vec<_>, _>>().map_err(|e| anyhow::anyhow!("{:?}", e))
}

pub async fn serve_mtls(
    app: Router,
    addr: std::net::SocketAddr,
    cfg: &Config,
) -> anyhow::Result<()> {
    let server_cert_path = cfg
        .mtls_server_cert
        .as_ref()
        .map(|p| cfg.resolve_path(p))
        .unwrap_or_else(|| cfg.work_dir.join("certs-mtls").join("server-cert.pem"));
    let server_key_path = cfg
        .mtls_server_key
        .as_ref()
        .map(|p| cfg.resolve_path(p))
        .unwrap_or_else(|| cfg.work_dir.join("certs-mtls").join("server-key.pem"));
    let ca_path = cfg
        .mtls_ca_cert
        .as_ref()
        .map(|p| cfg.resolve_path(p))
        .unwrap_or_else(|| cfg.work_dir.join("certs-mtls").join("ca.pem"));

    let certs = load_cert(&server_cert_path)?;
    let key = load_private_key(&server_key_path)?;
    let ca_certs = load_ca_certs(&ca_path)?;

    let mut root_store = rustls::RootCertStore::empty();
    for cert in ca_certs {
        root_store.add(cert).map_err(|e| anyhow::anyhow!("CA cert: {}", e))?;
    }

    let client_verifier = WebPkiClientVerifier::builder(Arc::new(root_store))
        .build()
        .map_err(|e| anyhow::anyhow!("client verifier: {}", e))?;

    let mut config = rustls::ServerConfig::builder()
        .with_client_cert_verifier(client_verifier)
        .with_single_cert(certs, key)
        .map_err(|e| anyhow::anyhow!("Server cert: {}", e))?;

    config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];

    let rustls_config = RustlsConfig::from_config(Arc::new(config));
    axum_server::bind_rustls(addr, rustls_config)
        .serve(app.into_make_service())
        .await
        .map_err(|e| anyhow::anyhow!("serve_mtls: {}", e))?;
    Ok(())
}
