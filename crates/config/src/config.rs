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
    pub url: String,
    pub user: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct RedisConfig {
    pub url: String,
}

type ServerBindingConfig = BindingConfig;

#[derive(Debug, Clone)]
pub struct PrometheusConfig {
    pub ip: IpAddr,
    pub port: u16,
    pub path: String,
}

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
    pub redis: RedisConfig,
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
                path: value.prometheus_path,
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
                url: value.s3_url,
                user: value.s3_user,
                password: value.s3_password,
            },
            redis: RedisConfig {
                url: value.redis_url,
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
        // Errors: Incompatible config, these return ConfigParsingError

        // Warnings: Ignored or deprecated configs
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

    impl Default for CliConfig {
        fn default() -> Self {
            CliConfig {
                config: None,
                debug: false,
                ip: LOCALHOST,
                port: DEFAULT_PORT,
                s3_url: DEFAULT_S3_URL.to_string(),
                s3_user: DEFAULT_S3_USER.to_string(),
                s3_password: DEFAULT_S3_PASSWORD.to_string(),
                redis_url: DEFAULT_REDIS_URL.to_string(),
                database_host: DEFAULT_DATABASE_HOST.to_string(),
                database_port: DEFAULT_DATABASE_PORT,
                database_name: DEFAULT_DATABASE_NAME.to_string(),
                database_user: DEFAULT_DATABASE_USER.to_string(),
                database_password: DEFAULT_DATABASE_PASSWORD.to_string(),
                prometheus_ip: DEFAULT_PROMETHEUS_IP,
                prometheus_port: DEFAULT_PROMETHEUS_PORT,
                prometheus_path: DEFAULT_PROMETHEUS_PATH.to_string(),
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
