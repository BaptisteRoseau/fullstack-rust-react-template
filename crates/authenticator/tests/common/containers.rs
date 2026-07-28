use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;

use async_trait::async_trait;
use serde_json::Value;
use testcontainers::core::ContainerPort::Tcp;
use testcontainers::core::WaitFor;
use testcontainers::{ContainerAsync, GenericImage, ImageExt, runners::AsyncRunner};
use tokio::sync::RwLock;

use authenticator::backends::Keycloak;
use cache::Cache;
use cache::error::CacheError;
use config::{
    ApiConfig, AuthenticatorConfig, BindingConfig, Config, PostgresConfig, RedisConfig,
    S3Config,
};
use database::Database;
use database::testing::MockDatabase;

const KEYCLOAK_IMAGE: &str = "quay.io/keycloak/keycloak";
const KEYCLOAK_TAG: &str = "26.6.4";
const KEYCLOAK_PORT: u16 = 8080;

pub const REALM: &str = "test-realm";
pub const CLIENT_ID: &str = "backend";
pub const USERNAME: &str = "testuser";
pub const PASSWORD: &str = "testpass";
/// Fixed user id declared in `assets/realm-export.json`, mirrored back as the
/// JWT `sub` claim so tests can assert on the resolved [`UserToken`] id.
pub const USER_ID: &str = "11111111-1111-1111-1111-111111111111";

/// Realm imported on container startup. Provides the `backend` client (direct
/// access grants + audience mapper) and the `testuser` account.
const REALM_EXPORT: &str = include_str!("../assets/realm-export.json");

pub struct KeycloakFixture {
    #[allow(dead_code)]
    container: ContainerAsync<GenericImage>,
    pub base_url: String,
}

impl KeycloakFixture {
    pub async fn start() -> Self {
        let container = GenericImage::new(KEYCLOAK_IMAGE, KEYCLOAK_TAG)
            .with_exposed_port(Tcp(KEYCLOAK_PORT))
            .with_wait_for(WaitFor::message_on_stdout("started in"))
            .with_cmd(["start-dev", "--import-realm"])
            .with_env_var("KC_BOOTSTRAP_ADMIN_USERNAME", "admin")
            .with_env_var("KC_BOOTSTRAP_ADMIN_PASSWORD", "admin")
            .with_env_var("KEYCLOAK_ADMIN", "admin")
            .with_env_var("KEYCLOAK_ADMIN_PASSWORD", "admin")
            .with_copy_to(
                "/opt/keycloak/data/import/realm-export.json",
                REALM_EXPORT.as_bytes().to_vec(),
            )
            .start()
            .await
            .expect("failed to start keycloak container");

        let port = container
            .get_host_port_ipv4(KEYCLOAK_PORT)
            .await
            .expect("failed to get keycloak http port");

        Self {
            container,
            base_url: format!("http://127.0.0.1:{port}"),
        }
    }

    /// JWKS endpoint the [`Keycloak`] authenticator fetches signing keys from.
    pub fn provider_url(&self) -> String {
        format!(
            "{}/realms/{REALM}/protocol/openid-connect/certs",
            self.base_url
        )
    }

    /// Builds a real [`Keycloak`] authenticator pointed at this container.
    ///
    /// The cache and database are no-ops: the JWT path never touches them, and
    /// the API-key path only needs the database to report "not found".
    pub async fn authenticator(&self) -> Keycloak {
        let config = test_config(self.provider_url());
        let cache: Arc<RwLock<dyn Cache>> = Arc::new(RwLock::new(NoopCache));
        let database: Arc<RwLock<dyn Database>> =
            Arc::new(RwLock::new(MockDatabase::default()));
        Keycloak::try_new(&config, cache, database)
            .await
            .expect("failed to build keycloak authenticator")
    }

    /// Obtains a fresh access token for `testuser` via the direct access grant.
    pub async fn fetch_token(&self) -> String {
        let url = format!(
            "{}/realms/{REALM}/protocol/openid-connect/token",
            self.base_url
        );
        // Credentials are alphanumeric, so no URL-encoding is required.
        let body = format!(
            "grant_type=password&client_id={CLIENT_ID}&username={USERNAME}\
             &password={PASSWORD}&scope=openid"
        );

        let response = reqwest::Client::new()
            .post(&url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("token request failed");
        let status = response.status();
        let body: Value = response.json().await.unwrap_or_else(|e| {
            panic!("token response was not JSON (status {status}): {e}")
        });

        body["access_token"]
            .as_str()
            .unwrap_or_else(|| {
                panic!("no access_token in token response (status {status}): {body}")
            })
            .to_string()
    }
}

fn test_config(provider_url: String) -> Config {
    Config {
        debug: false,
        log_json: false,
        api: ApiConfig {
            timeout_sec: 30,
            rate_limiter_refresh_per_second: 1,
            rate_limiter_burst_size: 1,
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
        oidc: config::OidcConfig {
            issuer_url: String::new(),
            client_id: String::new(),
            client_secret: String::new(),
            redirect_url: String::new(),
            frontend_url: String::new(),
            cookie_secure: false,
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
            provider_url,
            audiences: vec![CLIENT_ID.to_string()],
        },
    }
}

/// Cache that stores nothing; the JWT validation path never reads or writes it.
struct NoopCache;

#[async_trait]
impl Cache for NoopCache {
    async fn set(
        &self,
        _key: &str,
        _value: &Value,
        _timeout_s: Option<u32>,
    ) -> Result<(), CacheError> {
        Ok(())
    }
    async fn get(&self, _key: &str) -> Result<Option<Value>, CacheError> {
        Ok(None)
    }
    async fn delete(&self, _key: &str) -> Result<(), CacheError> {
        Ok(())
    }
    async fn set_many(
        &self,
        _mappings: &HashMap<String, Value>,
        _timeout_s: Option<u32>,
    ) -> Result<(), CacheError> {
        Ok(())
    }
    async fn get_many(
        &self,
        _keys: &[&str],
    ) -> Result<HashMap<String, Value>, CacheError> {
        Ok(HashMap::new())
    }
    async fn delete_many(&self, _keys: &[&str]) -> Result<(), CacheError> {
        Ok(())
    }
}
