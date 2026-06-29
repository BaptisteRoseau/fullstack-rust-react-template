use std::net::{IpAddr, Ipv4Addr};

pub(crate) const LOCALHOST: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
pub(crate) const DEFAULT_PORT: u16 = 8080;

pub(crate) const DEFAULT_SWAGGER_UI_PATH: &str = "/v1/docs/swagger-ui";
pub(crate) const DEFAULT_OPENAPI_PATH: &str = "/v1/docs/openapi.json";

pub(crate) const DEFAULT_PROMETHEUS_IP: IpAddr = LOCALHOST;
pub(crate) const DEFAULT_PROMETHEUS_PORT: u16 = 9100;
pub(crate) const DEFAULT_PROMETHEUS_PATH: &str = "/metrics";

pub(crate) const DEFAULT_DATABASE_HOST: &str = "127.0.0.1";
pub(crate) const DEFAULT_DATABASE_PORT: u16 = 5432;
pub(crate) const DEFAULT_DATABASE_NAME: &str = "backend";
pub(crate) const DEFAULT_DATABASE_USER: &str = "backend";
pub(crate) const DEFAULT_DATABASE_PASSWORD: &str = "password";

pub(crate) const DEFAULT_S3_URL: &str = "http://127.0.0.1:9000";
pub(crate) const DEFAULT_S3_USER: &str = "backend";
pub(crate) const DEFAULT_S3_PASSWORD: &str = "password";

pub(crate) const DEFAULT_REDIS_URL: &str = "redis://127.0.0.1:6379";

pub(crate) const DEFAULT_API_TIMEOUT_SEC: u16 = 20;

pub(crate) const DEFAULT_RATE_LIMITER_REFRESH_PER_SECOND: u64 = 1;
pub(crate) const DEFAULT_RATE_LIMITER_BURST_SIZE: u32 = 100;

pub(crate) const DEFAULT_CONFIG_FILE_PATH: &str = ".config.yaml";

// Keycloak default JWKS endpoint (from docker-compose.authentication.yml: port 8090 → 8080)
pub(crate) const DEFAULT_AUTHENTICATOR_PROVIDER_URL: &str =
    "http://localhost:8090/realms/app/protocol/openid-connect/certs";
pub(crate) const DEFAULT_AUTHENTICATOR_AUDIENCES: &str = "backend";

// OIDC / OAuth Backend-for-Frontend (Authorization Code + PKCE against Keycloak).
pub(crate) const DEFAULT_OIDC_ISSUER_URL: &str = "http://localhost:8090/realms/app";
pub(crate) const DEFAULT_OIDC_CLIENT_ID: &str = "webapp";
// Dev-only secret; matches the `webapp` client in the imported realm. Override in production.
pub(crate) const DEFAULT_OIDC_CLIENT_SECRET: &str = "webapp-secret";
pub(crate) const DEFAULT_OIDC_REDIRECT_URL: &str =
    "http://localhost:8080/api/auth/callback";
pub(crate) const DEFAULT_FRONTEND_URL: &str = "http://localhost:3000";
pub(crate) const DEFAULT_COOKIE_SECURE: bool = false;
