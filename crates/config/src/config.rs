use crate::error::ConfigParsingError;
use std::net::IpAddr;
use tracing::warn;

use crate::cli::CliConfig;
use crate::defaults::*;

#[derive(Debug, Clone)]
pub struct ApiConfig {
    pub timeout_sec: u16,
    pub rate_limiter_refresh_per_second: u64,
    pub rate_limiter_burst_size: u32,
    /// Frontend origin: post-login redirect target and the allowed CORS origin.
    pub frontend_url: String,
    /// Whether auth cookies carry the `Secure` attribute (enable behind HTTPS).
    pub cookie_secure: bool,
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

#[derive(Debug, Clone)]
pub struct AuthenticatorConfig {
    /// Realm base URL, e.g. `http://localhost:8090/realms/app`. Every provider
    /// endpoint (JWKS, authorize, token, logout, userinfo) is derived from it.
    pub issuer_url: String,
    /// Audiences the access token must carry.
    pub audiences: Vec<String>,
    /// Confidential client used by the Backend-for-Frontend.
    pub client_id: String,
    pub client_secret: String,
    /// Backend callback URL registered as a redirect URI on the client.
    pub redirect_url: String,
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
    pub log_json: bool,
    pub api: ApiConfig,
    pub server: ServerBindingConfig,
    pub s3: S3Config,
    pub redis: RedisConfig,
    pub postgres: PostgresConfig,
    pub prometheus: Option<PrometheusConfig>,
    pub swagger: Option<SwaggerConfig>,
    pub authenticator: AuthenticatorConfig,
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
                swagger_ui_path: value.swagger_ui_path,
                openapi_path: value.swagger_openapi_path,
            })
        };

        Ok(Self {
            debug: value.debug,
            log_json: value.log_json,
            api: ApiConfig {
                timeout_sec: value.api_timeout_sec,
                rate_limiter_refresh_per_second: value.rate_limiter_refresh_per_second,
                rate_limiter_burst_size: value.rate_limiter_burst_size,
                frontend_url: value.frontend_url,
                cookie_secure: value.cookie_secure,
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
            authenticator: AuthenticatorConfig {
                issuer_url: value.authenticator_issuer_url,
                audiences: value
                    .authenticator_audiences
                    .split(',')
                    .filter(|s| !s.is_empty())
                    .map(str::to_string)
                    .collect(),
                client_id: value.authenticator_client_id,
                client_secret: value.authenticator_client_secret,
                redirect_url: value.authenticator_redirect_url,
            },
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
        if cli_config.no_prometheus
            && (cli_config.prometheus_ip != DEFAULT_PROMETHEUS_IP
                || cli_config.prometheus_port != DEFAULT_PROMETHEUS_PORT)
        {
            warn!("Ignoring Prometheus server configuration because it is deactivated.");
        }

        Ok(())
    }
}

test_utils::tests_file!(
    #[allow(clippy::field_reassign_with_default)]
    "_tests/test_config.rs"
);
