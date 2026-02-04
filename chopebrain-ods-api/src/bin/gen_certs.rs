//! Gera CA, certificado do servidor e do cliente para mTLS.
//! Salva em ./certs-mtls/ (ou CERT_OUTPUT_DIR) na raiz do repositório.

use rcgen::{
    BasicConstraints, Certificate, CertificateParams, DnType, DnValue::PrintableString,
    ExtendedKeyUsagePurpose, IsCa, KeyUsagePurpose,
};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use time::{Duration, OffsetDateTime};

fn main() -> anyhow::Result<()> {
    let out_dir = output_dir()?;
    fs::create_dir_all(&out_dir)?;
    println!("Gerando certificados em: {}", out_dir.display());

    let (not_before, not_after) = validity_period();

    // 1. CA autoassinada
    let ca = new_ca(not_before, not_after);
    let ca_pem = ca.serialize_pem()?;
    write_file(&out_dir.join("ca.pem"), &ca_pem)?;
    println!("  ca.pem");

    // 2. Certificado do servidor (localhost, 127.0.0.1) assinado pela CA
    let server_cert = new_server_cert(not_before, not_after);
    let server_cert_pem = server_cert.serialize_pem_with_signer(&ca)?;
    let server_key_pem = server_cert.serialize_private_key_pem();
    write_file(&out_dir.join("server-cert.pem"), &server_cert_pem)?;
    write_file(&out_dir.join("server-key.pem"), &server_key_pem)?;
    println!("  server-cert.pem");
    println!("  server-key.pem");

    // 3. Certificado do cliente assinado pela CA
    let client_cert = new_client_cert(not_before, not_after);
    let client_cert_pem = client_cert.serialize_pem_with_signer(&ca)?;
    let client_key_pem = client_cert.serialize_private_key_pem();
    write_file(&out_dir.join("client-cert.pem"), &client_cert_pem)?;
    write_file(&out_dir.join("client-key.pem"), &client_key_pem)?;
    println!("  client-cert.pem");
    println!("  client-key.pem");

    println!("Pronto. Use client-cert.pem e client-key.pem no Postman (Client Certificate).");
    Ok(())
}

fn output_dir() -> anyhow::Result<PathBuf> {
    if let Ok(dir) = std::env::var("CERT_OUTPUT_DIR") {
        return Ok(PathBuf::from(dir));
    }
    let work_dir = chopebrain_ods_api::config::find_env_dir()
        .map_err(|e| anyhow::anyhow!("Não foi possível encontrar o diretório do .env: {}", e))?;
    Ok(work_dir.join("certs-mtls"))
}

fn validity_period() -> (OffsetDateTime, OffsetDateTime) {
    let ten_years = Duration::new(365 * 10 * 86400, 0);
    let not_before = OffsetDateTime::now_utc().checked_sub(Duration::new(86400, 0)).unwrap();
    let not_after = OffsetDateTime::now_utc().checked_add(ten_years).unwrap();
    (not_before, not_after)
}

fn new_ca(not_before: OffsetDateTime, not_after: OffsetDateTime) -> Certificate {
    let mut params = CertificateParams::new(Vec::<String>::default());
    params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
    params
        .distinguished_name
        .push(DnType::CountryName, PrintableString("BR".into()));
    params
        .distinguished_name
        .push(DnType::OrganizationName, "ChopeBrain ODS mTLS");
    params
        .distinguished_name
        .push(DnType::CommonName, "ChopeBrain ODS CA");
    params.key_usages.push(KeyUsagePurpose::DigitalSignature);
    params.key_usages.push(KeyUsagePurpose::KeyCertSign);
    params.key_usages.push(KeyUsagePurpose::CrlSign);
    params.not_before = not_before;
    params.not_after = not_after;
    Certificate::from_params(params).expect("CA params")
}

fn new_server_cert(not_before: OffsetDateTime, not_after: OffsetDateTime) -> Certificate {
    let mut params = CertificateParams::new(vec![
        "localhost".to_string(),
        "127.0.0.1".to_string(),
    ]);
    params.distinguished_name.push(DnType::CommonName, "localhost");
    params.use_authority_key_identifier_extension = true;
    params.key_usages.push(KeyUsagePurpose::DigitalSignature);
    params
        .extended_key_usages
        .push(ExtendedKeyUsagePurpose::ServerAuth);
    params.not_before = not_before;
    params.not_after = not_after;
    Certificate::from_params(params).expect("server cert params")
}

fn new_client_cert(not_before: OffsetDateTime, not_after: OffsetDateTime) -> Certificate {
    let mut params = CertificateParams::new(vec!["client.mtls.chopebrain".to_string()]);
    params
        .distinguished_name
        .push(DnType::CommonName, "ChopeBrain API Client");
    params.use_authority_key_identifier_extension = true;
    params.key_usages.push(KeyUsagePurpose::DigitalSignature);
    params
        .extended_key_usages
        .push(ExtendedKeyUsagePurpose::ClientAuth);
    params.not_before = not_before;
    params.not_after = not_after;
    Certificate::from_params(params).expect("client cert params")
}

fn write_file(path: &std::path::Path, contents: &str) -> anyhow::Result<()> {
    let mut f = fs::File::create(path)?;
    f.write_all(contents.as_bytes())?;
    Ok(())
}
