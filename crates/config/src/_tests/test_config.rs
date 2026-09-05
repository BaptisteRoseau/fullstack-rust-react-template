//TODO: Config priority: default->file->env->cli
//TODO: CliConfig merging priority: self->other
use super::*;
use clap::Parser;
use std::net::{IpAddr, Ipv4Addr};

impl Default for CliConfig {
    fn default() -> Self {
        CliConfig {
            config: None,
            debug: false,
            log_json: false,
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
            rate_limiter_refresh_per_second: DEFAULT_RATE_LIMITER_REFRESH_PER_SECOND,
            rate_limiter_burst_size: DEFAULT_RATE_LIMITER_BURST_SIZE,
            no_prometheus: false,
            swagger_ui_path: DEFAULT_SWAGGER_UI_PATH.to_string(),
            swagger_openapi_path: DEFAULT_OPENAPI_PATH.to_string(),
            no_swagger: false,
            authenticator_audiences: DEFAULT_AUTHENTICATOR_AUDIENCES.to_string(),
            authenticator_issuer_url: DEFAULT_AUTHENTICATOR_ISSUER_URL.to_string(),
            authenticator_client_id: DEFAULT_AUTHENTICATOR_CLIENT_ID.to_string(),
            authenticator_client_secret: DEFAULT_AUTHENTICATOR_CLIENT_SECRET.to_string(),
            authenticator_redirect_url: DEFAULT_AUTHENTICATOR_REDIRECT_URL.to_string(),
            frontend_url: DEFAULT_FRONTEND_URL.to_string(),
            cookie_secure: DEFAULT_COOKIE_SECURE,
            mcp_path: DEFAULT_MCP_PATH.to_string(),
            mcp_allowed_hosts: DEFAULT_MCP_ALLOWED_HOSTS.to_string(),
            no_mcp_json_response: false,
            no_mcp: false,
        }
    }
}

#[test]
fn test_validate_ignore_swagger_config() {
    let mut cli_config = CliConfig::default();
    cli_config.no_swagger = true;

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

#[test]
fn test_validate_ignore_mcp_config() {
    let mut cli_config = CliConfig::default();
    cli_config.no_mcp = true;
    cli_config.mcp_path = "/somewhere-else".to_string();

    let result = Config::validate(&cli_config);
    let config = Config::try_from(cli_config);

    assert!(result.is_ok(), "validate rejected a disabled MCP server: {result:?}");
    let config = config.expect("building a Config with the MCP server disabled");
    assert!(
        config.mcp.is_none(),
        "expected no McpConfig when --no-mcp is set, got {:?}",
        config.mcp
    );
}

#[test]
fn test_mcp_allowed_hosts_are_split_on_commas() {
    let mut cli_config = CliConfig::default();
    cli_config.mcp_allowed_hosts = "localhost, api.example.com ,".to_string();

    let config = Config::try_from(cli_config).expect("building a Config with an MCP server");
    let mcp = config.mcp.expect("the MCP server is enabled by default");

    assert_eq!(
        mcp.allowed_hosts,
        vec!["localhost".to_string(), "api.example.com".to_string()],
        "empty and padded entries should be dropped, got {:?}",
        mcp.allowed_hosts
    );
}

#[test]
fn test_mcp_flags_are_exposed_on_the_command_line() {
    let cli_config = CliConfig::try_parse_from([
        "backend",
        "--mcp-path",
        "/tools",
        "--mcp-allowed-hosts",
        "api.example.com",
        "--no-mcp-json-response",
    ])
    .expect("the MCP flags should be accepted by the parser");

    let config = Config::try_from(cli_config).expect("building a Config");
    let mcp = config.mcp.expect("the MCP server is enabled by default");

    assert_eq!(mcp.path, "/tools", "--mcp-path did not reach Config, got {}", mcp.path);
    assert_eq!(
        mcp.allowed_hosts,
        vec!["api.example.com".to_string()],
        "--mcp-allowed-hosts did not reach Config, got {:?}",
        mcp.allowed_hosts
    );
    assert!(
        !mcp.json_response,
        "--no-mcp-json-response did not reach Config, got {}",
        mcp.json_response
    );
}
