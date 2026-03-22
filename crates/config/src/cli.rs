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

    /// The IP where to bind the server
    #[arg(short, long, env, default_value_t = LOCALHOST)]
    pub(crate) ip: IpAddr,

    /// The port where to bind the server
    #[arg(short, long, env, default_value_t = DEFAULT_PORT)]
    pub(crate) port: u16,

    /// The PEM private key file used for HTTPS and JWT.
    /// If not provided, default to HTTP and static token for JWT.
    /// Required in production.
    #[arg(long, env)]
    pub(crate) pem_priv_key: Option<PathBuf>,

    /// The PEM public key file used for HTTPS and JWT
    /// If not provided, default to HTTP and static token for JWT.
    /// Required in production.
    #[arg(long, env)]
    pub(crate) pem_pub_key: Option<PathBuf>,

    /// JSON Web Token (JWT) Time To Live (TTL) in seconds.
    /// Users will be required to log back in after the
    /// Default is 3 month.
    #[arg(long, env, default_value_t = 8035200)]
    pub(crate) jwt_ttl_s: i64,

    /// Salt used on used passwords before hashing and storing
    /// into database.
    #[arg(env, default_value_t = String::from(DEFAULT_SALT))]
    pub(crate) password_salt: String,

    /// Timeout of the API in seconds. Use 0 for no timeout.
    #[arg(env, default_value_t = u16::from(DEFAULT_API_TIMEOUT_SEC))]
    pub(crate) api_timeout_sec: u16,

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
    /// Database host
    #[arg(long, env, default_value_t = DEFAULT_S3_HOST.to_string())]
    pub(crate) s3_host: String,

    /// Database port
    #[arg(long, env, default_value_t = DEFAULT_S3_PORT)]
    pub(crate) s3_port: u16,

    /// Database user
    #[arg(long, env, default_value_t = DEFAULT_S3_USER.to_string())]
    pub(crate) s3_user: String,

    /// Database password
    #[arg(long, env, default_value_t = DEFAULT_S3_PASSWORD.to_string())]
    pub(crate) s3_password: String,

    /* ===============
    PROMETHEUS
    ================ */
    /// Prometheus server host
    #[arg(long, env, default_value_t = DEFAULT_PROMETHEUS_IP)]
    pub(crate) prometheus_ip: IpAddr,

    /// Prometheus server port
    #[arg(long, env, default_value_t = DEFAULT_PROMETHEUS_PORT)]
    pub(crate) prometheus_port: u16,

    /// Deactivate Prometheus metric server
    #[arg(long, env, default_value_t = false)]
    pub(crate) no_prometheus: bool,

    /* ===============
    SWAGGER
    ================ */
    /// The IP where to bind the swagger server
    #[arg(long, env, default_value_t = DEFAULT_SWAGGER_IP)]
    pub(crate) swagger_ip: IpAddr,

    /// The port where to bind the swagger server
    #[arg(long, env, default_value_t = DEFAULT_SWAGGER_PORT)]
    pub(crate) swagger_port: u16,

    /// The path where to bind the swagger server
    #[arg(long, env, default_value_t = DEFAULT_SWAGGER_UI_PATH.to_string())]
    pub(crate) swagger_ui_path: String,

    /// The path where to bind the swagger server
    #[arg(long, env, default_value_t = DEFAULT_OPENAPI_PATH.to_string())]
    pub(crate) swagger_openapi_path: String,

    /// Deactivate Swagger server
    #[arg(long, env, default_value_t = false)]
    pub(crate) no_swagger: bool,
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
