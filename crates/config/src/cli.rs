use crate::defaults::*;
use crate::error::ConfigParsingError;
use clap::Parser;
use serde::Deserialize;
use std::fs;
use std::net::IpAddr;
use std::path::{Path, PathBuf};
use tracing::warn;

/* ======================================================================================
FULL CONFIG FROM USER
====================================================================================== */

/// Backend server configuration.
///
/// This struct serves as a parser for the configuration file and command line arguments.
/// It is then parsed to build the full configuration [`Config`] for the server.
///
/// This is done like this to keep all arguments available via the configuration file or
/// CLI, while allowing the [`Config`]'s substructures to be valid for the rest of the program.
/// For example, the [`PrometheusConfig`] and [`PostgresConfig`] will be built if and only
/// if all their parameters are provided, hence no need to check each of them in the client code.
///
/// CLI arguments grouped together into a single struct should be prefixed with the same
/// name.
/// For example, all arguments related to the database should be prefixed with `database_`.
#[derive(Parser, Deserialize, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub(crate) struct CliConfig {
    /// Path to the configuration file
    #[arg(short, long, env)]
    pub(crate) config: Option<PathBuf>,

    /// Enable debug logging
    #[arg(long, env, default_value_t = false)]
    pub(crate) debug: bool,

    /// Output logs in JSON format instead of the human-readable compact format
    #[arg(long, env, default_value_t = false)]
    pub(crate) log_json: bool,

    /// The IP where to bind the server
    #[arg(short, long, env, default_value_t = LOCALHOST)]
    pub(crate) ip: IpAddr,

    /// The port where to bind the server
    #[arg(short, long, env, default_value_t = DEFAULT_PORT)]
    pub(crate) port: u16,

    /// Timeout of the API in seconds. Use 0 for no timeout.
    #[arg(env, default_value_t = u16::from(DEFAULT_API_TIMEOUT_SEC))]
    pub(crate) api_timeout_sec: u16,

    /// Number of seconds after which one request slot is replenished by the rate limiter.
    #[arg(long, env, default_value_t = DEFAULT_RATE_LIMITER_REFRESH_PER_SECOND)]
    pub(crate) rate_limiter_refresh_per_second: u64,

    /// Maximum number of requests allowed in a burst before the rate limiter kicks in.
    #[arg(long, env, default_value_t = DEFAULT_RATE_LIMITER_BURST_SIZE)]
    pub(crate) rate_limiter_burst_size: u32,

    /// Frontend base URL to redirect the browser to after login
    #[arg(long, env, default_value_t = DEFAULT_FRONTEND_URL.to_string())]
    pub(crate) frontend_url: String,

    /// Whether auth cookies should set the Secure attribute (enable in production/HTTPS)
    #[arg(long, env, default_value_t = DEFAULT_COOKIE_SECURE)]
    pub(crate) cookie_secure: bool,

    /* ===============
    DATABASE
    ================ */
    /// S3 host
    #[arg(long, env, default_value_t = DEFAULT_DATABASE_HOST.to_string())]
    pub(crate) database_host: String,

    /// S3 port
    #[arg(long, env, default_value_t = DEFAULT_DATABASE_PORT)]
    pub(crate) database_port: u16,

    /// S3 name
    #[arg(long, env, default_value_t = DEFAULT_DATABASE_NAME.to_string())]
    pub(crate) database_name: String,

    /// S3 user
    #[arg(long, env, default_value_t = DEFAULT_DATABASE_USER.to_string())]
    pub(crate) database_user: String,

    /// S3 password
    #[arg(long, env, default_value_t = DEFAULT_DATABASE_PASSWORD.to_string())]
    pub(crate) database_password: String,

    /* ===============
    S3
    ================ */
    /// Database url
    #[arg(long, env, default_value_t = DEFAULT_S3_URL.to_string())]
    pub(crate) s3_url: String,

    /// Database user
    #[arg(long, env, default_value_t = DEFAULT_S3_USER.to_string())]
    pub(crate) s3_user: String,

    /// Database password
    #[arg(long, env, default_value_t = DEFAULT_S3_PASSWORD.to_string())]
    pub(crate) s3_password: String,

    /* ===============
    REDIS
    ================ */
    /// Redis URL (e.g. redis://127.0.0.1:6379)
    #[arg(long, env, default_value_t = DEFAULT_REDIS_URL.to_string())]
    pub(crate) redis_url: String,

    /* ===============
    PROMETHEUS
    ================ */
    /// Prometheus server host
    #[arg(long, env, default_value_t = DEFAULT_PROMETHEUS_IP)]
    pub(crate) prometheus_ip: IpAddr,

    /// Prometheus server port
    #[arg(long, env, default_value_t = DEFAULT_PROMETHEUS_PORT)]
    pub(crate) prometheus_port: u16,

    /// Prometheus metrics endpoint
    #[arg(long, env, default_value_t = DEFAULT_PROMETHEUS_PATH.to_string())]
    pub(crate) prometheus_path: String,

    /// Deactivate Prometheus metric server
    #[arg(long, env, default_value_t = false)]
    pub(crate) no_prometheus: bool,

    /* ===============
    SWAGGER
    ================ */
    /// The path where to bind the swagger server
    #[arg(long, env, default_value_t = DEFAULT_SWAGGER_UI_PATH.to_string())]
    pub(crate) swagger_ui_path: String,

    /// The path where to bind the swagger server
    #[arg(long, env, default_value_t = DEFAULT_OPENAPI_PATH.to_string())]
    pub(crate) swagger_openapi_path: String,

    /// Deactivate Swagger server
    #[arg(long, env, default_value_t = false)]
    pub(crate) no_swagger: bool,

    /* ===============
    AUTHENTICATOR (JWT/API-key validation + OAuth Backend-for-Frontend, both driven
    by the same Keycloak realm)
    ================ */
    /// Comma-separated list of accepted JWT audiences
    #[arg(long, env, default_value_t = DEFAULT_AUTHENTICATOR_AUDIENCES.to_string())]
    pub(crate) authenticator_audiences: String,

    /// Authenticator issuer base URL (e.g. http://localhost:8090/realms/app). Every
    /// provider endpoint (JWKS, authorize, token, logout, userinfo) is derived from it.
    #[arg(long, env, default_value_t = DEFAULT_AUTHENTICATOR_ISSUER_URL.to_string())]
    pub(crate) authenticator_issuer_url: String,

    /// Authenticator confidential client id used by the backend
    #[arg(long, env, default_value_t = DEFAULT_AUTHENTICATOR_CLIENT_ID.to_string())]
    pub(crate) authenticator_client_id: String,

    /// Authenticator confidential client secret
    #[arg(long, env, default_value_t = DEFAULT_AUTHENTICATOR_CLIENT_SECRET.to_string())]
    pub(crate) authenticator_client_secret: String,

    /// OAuth redirect URL pointing back to the backend callback endpoint
    #[arg(long, env, default_value_t = DEFAULT_AUTHENTICATOR_REDIRECT_URL.to_string())]
    pub(crate) authenticator_redirect_url: String,
}

impl CliConfig {
    /// Loads the configuration file and updates its value with the provided CLI/ENV arguments.
    ///
    /// The CLI/ENV arguments take precedence over the configuration file.
    pub fn parse_with_file() -> Result<CliConfig, ConfigParsingError> {
        let mut config: CliConfig = Self::parse();

        let mut file_config: Option<CliConfig> = None;
        if let Some(file) = &config.config {
            file_config = Some(serde_yaml::from_str(fs::read_to_string(file)?.as_str())?);
        } else if Path::new(DEFAULT_CONFIG_FILE_PATH).is_file() {
            file_config = Some(serde_yaml::from_str(
                fs::read_to_string(DEFAULT_CONFIG_FILE_PATH)?.as_str(),
            )?);
        }

        if let Some(file_config) = file_config {
            config = file_config.merge(config);
        }

        Ok(config)
    }

    /// Overwrites the current configuration with the provided one.
    fn merge(&self, other: CliConfig) -> CliConfig {
        // Add warnings for keys that are being overridden
        let _ = other;
        todo!("Config file not supported yet");
    }

    /// Generates a default configuration file template.
    #[allow(dead_code)]
    pub fn template() -> String {
        todo!(
            "Use clap to generate a default configuration template with documentation and commented defaults"
        );
    }
}
