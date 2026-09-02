//! Ready-made [`Config`] test double, shared by downstream crates' tests.
//!
//! Enable via the `test-utils` feature (add it under `[dev-dependencies]`, not
//! `[dependencies]`, so it never leaks into non-test builds).

use std::net::{IpAddr, Ipv4Addr};

use crate::config::{
    ApiConfig, AuthenticatorConfig, BindingConfig, Config, PostgresConfig, RedisConfig,
    S3Config, StorageConfig,
};
use crate::defaults::*;

/// A fully populated [`Config`] with inert values, for tests that need a
/// `Config` but only care about a few fields. Mutate the returned value as needed.
///
/// Connection details (s3, redis, postgres) are left empty so a test never
/// accidentally reaches a real service; `prometheus` and `swagger` are disabled;
/// the server binds to an ephemeral port on loopback.
pub fn test_config() -> Config {
    Config {
        debug: false,
        log_json: false,
        api: ApiConfig {
            timeout_sec: DEFAULT_API_TIMEOUT_SEC,
            rate_limiter_refresh_per_second: DEFAULT_RATE_LIMITER_REFRESH_PER_SECOND,
            rate_limiter_burst_size: DEFAULT_RATE_LIMITER_BURST_SIZE,
            frontend_url: DEFAULT_FRONTEND_URL.to_string(),
            cookie_secure: DEFAULT_COOKIE_SECURE,
        },
        server: BindingConfig {
            ip: IpAddr::V4(Ipv4Addr::LOCALHOST),
            port: 0,
        },
        s3: S3Config {
            url: String::new(),
            user: String::new(),
            password: String::new(),
        },
        storage: StorageConfig {
            encryption_key: [0u8; STORAGE_ENCRYPTION_KEY_LENGTH],
        },
        redis: RedisConfig { url: String::new() },
        postgres: PostgresConfig {
            host: String::new(),
            port: 0,
            database: String::new(),
            user: String::new(),
            password: String::new(),
        },
        prometheus: None,
        swagger: None,
        authenticator: AuthenticatorConfig {
            issuer_url: DEFAULT_AUTHENTICATOR_ISSUER_URL.to_string(),
            audiences: DEFAULT_AUTHENTICATOR_AUDIENCES
                .split(',')
                .filter(|s| !s.is_empty())
                .map(str::to_string)
                .collect(),
            client_id: DEFAULT_AUTHENTICATOR_CLIENT_ID.to_string(),
            client_secret: DEFAULT_AUTHENTICATOR_CLIENT_SECRET.to_string(),
            redirect_url: DEFAULT_AUTHENTICATOR_REDIRECT_URL.to_string(),
        },
    }
}
