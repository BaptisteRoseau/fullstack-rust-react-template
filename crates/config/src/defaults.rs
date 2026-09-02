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

/// Master key wrapping every per-file data encryption key, base64 of 32 bytes.
// Dev-only secret; anything encrypted under it is readable by anyone with this
// repository. Override in production.
pub(crate) const DEFAULT_STORAGE_ENCRYPTION_KEY: &str =
    "ZGV2LW9ubHktc3RvcmFnZS1tYXN0ZXIta2V5LTMyYiE=";

/// Number of bytes the decoded [`DEFAULT_STORAGE_ENCRYPTION_KEY`] must hold,
/// set by AES-256-GCM.
pub const STORAGE_ENCRYPTION_KEY_LENGTH: usize = 32;

pub(crate) const DEFAULT_API_TIMEOUT_SEC: u16 = 20;

pub(crate) const DEFAULT_RATE_LIMITER_REFRESH_PER_SECOND: u64 = 1;
pub(crate) const DEFAULT_RATE_LIMITER_BURST_SIZE: u32 = 100;

pub(crate) const DEFAULT_CONFIG_FILE_PATH: &str = ".config.yaml";

pub(crate) const DEFAULT_AUTHENTICATOR_AUDIENCES: &str = "backend";

// Authenticator: JWT/API-key validation + OAuth Backend-for-Frontend (Authorization
// Code + PKCE), both derived from the same Keycloak realm (from
// docker-compose.authentication.yml: port 8090 → 8080).
pub(crate) const DEFAULT_AUTHENTICATOR_ISSUER_URL: &str =
    "http://localhost:8090/realms/app";
pub(crate) const DEFAULT_AUTHENTICATOR_CLIENT_ID: &str = "webapp";
// Dev-only secret; matches the `webapp` client in the imported realm. Override in production.
pub(crate) const DEFAULT_AUTHENTICATOR_CLIENT_SECRET: &str = "webapp-secret";
pub(crate) const DEFAULT_AUTHENTICATOR_REDIRECT_URL: &str =
    "http://localhost:8080/api/auth/callback";
pub(crate) const DEFAULT_FRONTEND_URL: &str = "http://localhost:3000";
pub(crate) const DEFAULT_COOKIE_SECURE: bool = false;
