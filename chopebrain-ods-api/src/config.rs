//! Configuração carregada do .env na raiz do repositório.

use std::path::PathBuf;

/// Encontra o diretório raiz onde está o .env (mesmo usado pelo Python).
/// 1. Usa API_ENV_FILE se definido (caminho para um .env alternativo; work_dir = diretório do arquivo).
/// 2. Usa WORK_DIR se definido.
/// 3. Procura .env no diretório atual e em diretórios pais.
pub fn find_env_dir() -> Result<PathBuf, std::io::Error> {
    if let Ok(env_file) = std::env::var("API_ENV_FILE") {
        let p = PathBuf::from(&env_file);
        if p.is_file() {
            return Ok(p.parent().unwrap_or_else(|| std::path::Path::new(".")).to_path_buf());
        }
    }
    if let Ok(work_dir) = std::env::var("WORK_DIR") {
        let p = PathBuf::from(work_dir);
        if p.join(".env").is_file() {
            return Ok(p);
        }
    }
    let mut current = std::env::current_dir()?;
    loop {
        let env_path = current.join(".env");
        if env_path.is_file() {
            return Ok(current);
        }
        if !current.pop() {
            break;
        }
    }
    std::env::current_dir()
}

/// Carrega variáveis do .env da raiz do repo (não do diretório do crate).
/// Se API_ENV_FILE estiver definido, carrega esse arquivo.
pub fn load_dotenv_from_root() -> Result<PathBuf, dotenvy::Error> {
    let root = find_env_dir().map_err(|e| dotenvy::Error::Io(e))?;
    let env_path = std::env::var("API_ENV_FILE")
        .ok()
        .map(PathBuf::from)
        .filter(|p| p.is_file())
        .unwrap_or_else(|| root.join(".env"));
    dotenvy::from_path(&env_path)?;
    Ok(root)
}

#[derive(Clone, Debug)]
pub struct Config {
    pub ods_host: String,
    pub ods_port: u16,
    pub ods_user: String,
    pub ods_password: String,
    pub ods_name: String,
    pub ods_ssl_ca: Option<PathBuf>,
    pub jwt_secret: String,
    pub jwt_expiration_days: u64,
    pub auth_username: Option<String>,
    pub auth_password: Option<String>,
    pub auth_secret: Option<String>,
    pub mtls_server_cert: Option<PathBuf>,
    pub mtls_server_key: Option<PathBuf>,
    pub mtls_ca_cert: Option<PathBuf>,
    pub work_dir: PathBuf,
}

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct ConfigError(pub String);

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let work_dir = find_env_dir().map_err(|e| ConfigError(e.to_string()))?;
        let ods_host = std::env::var("ODS_HOST")
            .map_err(|_| ConfigError("ODS_HOST não definido".into()))?;
        let ods_port = std::env::var("ODS_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(3306);
        let ods_user = std::env::var("ODS_USER")
            .map_err(|_| ConfigError("ODS_USER não definido".into()))?;
        let ods_password = std::env::var("ODS_PASSWORD")
            .map_err(|_| ConfigError("ODS_PASSWORD não definido".into()))?;
        let ods_name = std::env::var("ODS_NAME")
            .map_err(|_| ConfigError("ODS_NAME não definido".into()))?;
        let ods_ssl_ca = std::env::var("ODS_SSL_CA").ok().map(PathBuf::from);
        let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "change-me-in-production".into());
        let jwt_expiration_days = std::env::var("JWT_EXPIRATION_DAYS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(7);
        let auth_username = std::env::var("AUTH_USERNAME").ok();
        let auth_password = std::env::var("AUTH_PASSWORD").ok();
        let auth_secret = std::env::var("AUTH_SECRET").ok();
        let mtls_server_cert = std::env::var("MTLS_SERVER_CERT").ok().map(PathBuf::from);
        let mtls_server_key = std::env::var("MTLS_SERVER_KEY").ok().map(PathBuf::from);
        let mtls_ca_cert = std::env::var("MTLS_CA_CERT").ok().map(PathBuf::from);

        Ok(Self {
            ods_host,
            ods_port,
            ods_user,
            ods_password,
            ods_name,
            ods_ssl_ca,
            jwt_secret,
            jwt_expiration_days,
            auth_username,
            auth_password,
            auth_secret,
            mtls_server_cert,
            mtls_server_key,
            mtls_ca_cert,
            work_dir,
        })
    }

    /// Resolve caminho relativo em relação ao work_dir (raiz do repo).
    pub fn resolve_path(&self, path: &std::path::Path) -> PathBuf {
        if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.work_dir.join(path)
        }
    }

    pub fn ods_ssl_ca_path(&self) -> Option<PathBuf> {
        self.ods_ssl_ca.as_ref().map(|p| self.resolve_path(p))
    }
}
