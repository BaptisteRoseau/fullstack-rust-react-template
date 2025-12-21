use crate::errors::ConfigParsingError;
use clap::Parser;
use serde::Deserialize;
use std::fs;
use std::net::{IpAddr, Ipv4Addr};
use std::path::{Path, PathBuf};
use tracing::warn;

const LOCALHOST: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
const DEFAULT_PORT: u16 = 6969;

const DEFAULT_SWAGGER_IP: IpAddr = LOCALHOST;
const DEFAULT_SWAGGER_PORT: u16 = 7070;

const DEFAULT_SWAGGER_UI_PATH: &str = "/v1/docs/swagger-ui";
const DEFAULT_OPENAPI_PATH: &str = "/v1/docs/openapi.json";

const DEFAULT_PROMETHEUS_IP: IpAddr = LOCALHOST;
const DEFAULT_PROMETHEUS_PORT: u16 = 9100;

const DEFAULT_DATABASE_HOST: &str = "127.0.0.1";
const DEFAULT_DATABASE_PORT: u16 = 5432;
const DEFAULT_DATABASE_NAME: &str = "backend";
const DEFAULT_DATABASE_USER: &str = "backend";
const DEFAULT_DATABASE_PASSWORD: &str = "password";

//TODO: Add MinIO support

const DEFAULT_CONFIG_FILE_PATH: &str = ".config.yaml";

const DEFAULT_SALT: &str = "This default if for development purposes only";

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
struct CliConfig {
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

    /* ===============
    DATABASE
    ================ */
    /// Database host
    #[arg(long, env, default_value_t = DEFAULT_DATABASE_HOST.to_string())]
    pub(crate) database_host: String,

    /// Database port
    #[arg(long, env, default_value_t = DEFAULT_DATABASE_PORT)]
    pub(crate) database_port: u16,

    /// Database name
    #[arg(long, env, default_value_t = DEFAULT_DATABASE_NAME.to_string())]
    pub(crate) database_name: String,

    /// Database user
    #[arg(long, env, default_value_t = DEFAULT_DATABASE_USER.to_string())]
    pub(crate) database_user: String,

    /// Database password
    #[arg(long, env, default_value_t = DEFAULT_DATABASE_PASSWORD.to_string())]
    pub(crate) database_password: String,

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
    pub(crate) fn parse_with_file() -> Result<CliConfig, ConfigParsingError> {
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
    pub(crate) fn template() -> String {
        todo!(
            "Use clap to generate a default configuration template with documentation and commented defaults"
        );
    }
}

/* ======================================================================================
CONFIG
====================================================================================== */

#[derive(Debug, Clone)]
pub(crate) struct BindingConfig {
    pub(crate) ip: IpAddr,
    pub(crate) port: u16,
}

#[derive(Debug, Clone)]
pub(crate) struct PostgresConfig {
    pub(crate) host: String,
    pub(crate) port: u16,
    pub(crate) database: String,
    pub(crate) user: String,
    pub(crate) password: String,
}

type ServerBindingConfig = BindingConfig;
type PrometheusConfig = BindingConfig;

#[derive(Debug, Clone)]
pub(crate) struct SwaggerConfig {
    pub(crate) ip: IpAddr,
    pub(crate) port: u16,
    pub(crate) swagger_ui_path: String,
    pub(crate) openapi_path: String,
}

/// The main configuration.
///
/// This struct is passed to the whole program to configure the server.
/// All of its attributes are considered valid and should be used as is if not None.
///
/// Any user input validation should be done within this struct,
/// in the [`Config::validate`] method.
#[derive(Debug, Clone)]
pub(crate) struct Config {
    pub(crate) debug: bool,
    pub(crate) server: ServerBindingConfig,
    pub(crate) postgres: PostgresConfig,
    pub(crate) prometheus: Option<PrometheusConfig>,
    pub(crate) swagger: Option<SwaggerConfig>,
}

impl Config {
    pub(crate) fn parse() -> Result<Self, ConfigParsingError> {
        Self::try_from(CliConfig::parse_with_file()?)
    }
}

impl TryFrom<CliConfig> for Config {
    type Error = ConfigParsingError;

    fn try_from(value: CliConfig) -> Result<Self, ConfigParsingError> {
        Self::validate(&value)?;

        let prometheus = if value.no_prometheus {
            None
        } else {
            Some(PrometheusConfig {
                ip: value.prometheus_ip,
                port: value.prometheus_port,
            })
        };

        let swagger = if value.no_swagger {
            None
        } else {
            Some(SwaggerConfig {
                ip: value.swagger_ip,
                port: value.swagger_port,
                swagger_ui_path: value.swagger_ui_path,
                openapi_path: value.swagger_openapi_path,
            })
        };

        Ok(Self {
            debug: value.debug,
            server: ServerBindingConfig {
                ip: value.ip,
                port: value.port,
            },
            postgres: PostgresConfig {
                host: value.database_host,
                port: value.database_port,
                database: value.database_name,
                user: value.database_user,
                password: value.database_password,
            },
            prometheus,
            swagger,
        })
    }
}

impl Config {
    /// Verifies the CLI configuration is valid, throw a [`ConfigParsingError`] is not.
    ///
    /// For example, makes sure the PEM key **AND** certificate are provided
    /// if the server is in production mode.
    fn validate(cli_config: &CliConfig) -> Result<(), ConfigParsingError> {
        // Errors
        if cli_config.pem_priv_key.is_some() && cli_config.pem_pub_key.is_none() {
            return Err(ConfigParsingError::MissingPemPubCert);
        }
        if cli_config.pem_priv_key.is_none() && cli_config.pem_pub_key.is_some() {
            return Err(ConfigParsingError::MissingPemPrivKey);
        }
        #[cfg(not(debug_assertions))]
        if cli_config.password_salt == String::from(DEFAULT_SALT) {
            return Err(ConfigParsingError::DefaultPasswordSaltInReleaseMode);
        }

        // Warnings
        if cli_config.no_swagger
            && (cli_config.swagger_ip != DEFAULT_SWAGGER_IP
                || cli_config.swagger_port != DEFAULT_SWAGGER_PORT)
        {
            warn!("Ignoring Swagger server configuration because it is deactivated.");
        }
        if cli_config.no_prometheus
            && (cli_config.prometheus_ip != DEFAULT_PROMETHEUS_IP
                || cli_config.prometheus_port != DEFAULT_PROMETHEUS_PORT)
        {
            warn!("Ignoring Prometheus server configuration because it is deactivated.");
        }

        Ok(())
    }
}

#[cfg(test)]
#[allow(clippy::field_reassign_with_default)]
mod test {
    //TODO: Config priority: default->file->env->cli
    //TODO: CliConfig merging priority: self->other
    use super::*;

    impl Default for CliConfig {
        fn default() -> Self {
            CliConfig {
                config: None,
                debug: false,
                ip: LOCALHOST,
                port: DEFAULT_PORT,
                pem_priv_key: None,
                pem_pub_key: None,
                jwt_ttl_s: 8035200,
                password_salt: String::from("For development purposes only"),
                database_host: DEFAULT_DATABASE_HOST.to_string(),
                database_port: DEFAULT_DATABASE_PORT,
                database_name: DEFAULT_DATABASE_NAME.to_string(),
                database_user: DEFAULT_DATABASE_USER.to_string(),
                database_password: DEFAULT_DATABASE_PASSWORD.to_string(),
                prometheus_ip: DEFAULT_PROMETHEUS_IP,
                prometheus_port: DEFAULT_PROMETHEUS_PORT,
                no_prometheus: false,
                swagger_ip: DEFAULT_SWAGGER_IP,
                swagger_port: DEFAULT_SWAGGER_PORT,
                swagger_ui_path: DEFAULT_SWAGGER_UI_PATH.to_string(),
                swagger_openapi_path: DEFAULT_OPENAPI_PATH.to_string(),
                no_swagger: false,
            }
        }
    }

    #[test]
    fn test_validate_missing_pem_pub_cert() {
        let mut cli_config = CliConfig::default();
        cli_config.pem_priv_key = Some(PathBuf::from("private_key.pem"));
        cli_config.pem_pub_key = None;

        let result = Config::validate(&cli_config);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConfigParsingError::MissingPemPubCert
        ));
    }

    #[test]
    fn test_validate_missing_pem_priv_key() {
        let mut cli_config = CliConfig::default();
        cli_config.pem_pub_key = Some(PathBuf::from("public_key.pem"));
        cli_config.pem_priv_key = None;

        let result = Config::validate(&cli_config);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConfigParsingError::MissingPemPrivKey
        ));
    }

    #[test]
    fn test_validate_ignore_swagger_config() {
        let mut cli_config = CliConfig::default();
        cli_config.no_swagger = true;
        cli_config.swagger_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        cli_config.swagger_port = 8080;

        let result = Config::validate(&cli_config);
        let config = Config::try_from(cli_config);

        assert!(result.is_ok());
        assert!(config.is_ok());
        assert!(config.unwrap().swagger.is_none());
    }

    #[test]
    fn test_validate_ignore_prometheus_config() {
        let mut cli_config = CliConfig::default();
        cli_config.no_prometheus = true;
        cli_config.prometheus_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));
        cli_config.prometheus_port = 9091;

        let result = Config::validate(&cli_config);
        let config = Config::try_from(cli_config);

        assert!(result.is_ok());
        assert!(config.is_ok());
        assert!(config.unwrap().prometheus.is_none());
    }
}
