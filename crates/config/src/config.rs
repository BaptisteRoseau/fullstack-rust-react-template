use crate::error::ConfigParsingError;
use std::net::IpAddr;
use tracing::warn;

use crate::cli::CliConfig;
use crate::defaults::*;

#[derive(Debug, Clone)]
pub struct ApiConfig {
    pub timeout_sec: u16,
}

#[derive(Debug, Clone)]
pub struct BindingConfig {
    pub ip: IpAddr,
    pub port: u16,
}

#[derive(Debug, Clone)]
pub struct PostgresConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub user: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct S3Config {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
}

type ServerBindingConfig = BindingConfig;
type PrometheusConfig = BindingConfig;

#[derive(Debug, Clone)]
pub struct SwaggerConfig {
    pub ip: IpAddr,
    pub port: u16,
    pub swagger_ui_path: String,
    pub openapi_path: String,
}

/// The main configuration.
///
/// This struct is passed to the whole program to configure the server.
/// All of its attributes are considered valid and should be used as is if not None.
///
/// Any user input validation should be done within this struct,
/// in the [`Config::validate`] method.
#[derive(Debug, Clone)]
pub struct Config {
    pub debug: bool,
    pub api: ApiConfig,
    pub server: ServerBindingConfig,
    pub s3: S3Config,
    pub postgres: PostgresConfig,
    pub prometheus: Option<PrometheusConfig>,
    pub swagger: Option<SwaggerConfig>,
}

impl Config {
    pub fn parse() -> Result<Self, ConfigParsingError> {
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
            api: ApiConfig {
                timeout_sec: value.api_timeout_sec,
            },
            server: ServerBindingConfig {
                ip: value.ip,
                port: value.port,
            },
            s3: S3Config {
                host: value.s3_host,
                port: value.s3_port,
                user: value.s3_user,
                password: value.s3_password,
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
    use std::net::{IpAddr, Ipv4Addr};
    use std::path::PathBuf;

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
                s3_host: DEFAULT_S3_HOST.to_string(),
                s3_port: DEFAULT_S3_PORT,
                s3_user: DEFAULT_S3_USER.to_string(),
                s3_password: DEFAULT_S3_PASSWORD.to_string(),
                database_host: DEFAULT_DATABASE_HOST.to_string(),
                database_port: DEFAULT_DATABASE_PORT,
                database_name: DEFAULT_DATABASE_NAME.to_string(),
                database_user: DEFAULT_DATABASE_USER.to_string(),
                database_password: DEFAULT_DATABASE_PASSWORD.to_string(),
                prometheus_ip: DEFAULT_PROMETHEUS_IP,
                prometheus_port: DEFAULT_PROMETHEUS_PORT,
                api_timeout_sec: DEFAULT_API_TIMEOUT_SEC,
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
