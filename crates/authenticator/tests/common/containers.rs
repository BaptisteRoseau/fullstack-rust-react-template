use std::sync::Arc;

use testcontainers::core::ContainerPort::Tcp;
use testcontainers::core::WaitFor;
use testcontainers::{ContainerAsync, GenericImage, ImageExt, runners::AsyncRunner};
use tokio::sync::RwLock;

use authenticator::backends::Keycloak;
use cache::Cache;
use cache::backends::hash_map::HashMapCache;
use config::Config;
use database::Database;
use database::testing::MockDatabase;
use test_trait::{Runtime, TestSuite, Trial};

const KEYCLOAK_IMAGE: &str = "quay.io/keycloak/keycloak";
/// Pinned: the login-form scraping in `common::provider` reads Keycloak's HTML,
/// so an unannounced upgrade must not silently change what the tests drive.
const KEYCLOAK_TAG: &str = "26.6.4";
const KEYCLOAK_PORT: u16 = 8080;

/// Audience both realms' mappers emit, and the only one the authenticators accept.
pub const AUDIENCE: &str = "backend";

/* ---------------------------------------------------------------------------
 * Credentials realm: exercises `validate`, so it only needs a public client with
 * direct access grants to mint a token without a browser.
 * ------------------------------------------------------------------------ */

pub const CREDENTIALS_REALM: &str = "test-realm";
pub const CREDENTIALS_CLIENT_ID: &str = "backend";
pub const CREDENTIALS_USERNAME: &str = "testuser";
pub const CREDENTIALS_PASSWORD: &str = "testpass";
/// Fixed user id declared in `assets/realm-export.json`, mirrored back as the
/// JWT `sub` claim so tests can assert on the resolved `UserToken` id.
pub const CREDENTIALS_USER_ID: &str = "11111111-1111-1111-1111-111111111111";

/* ---------------------------------------------------------------------------
 * Backend-for-Frontend realm: exercises the Authorization Code + PKCE flow, so
 * it needs a confidential client with the standard flow and registration on.
 * ------------------------------------------------------------------------ */

pub const BFF_REALM: &str = "oidc-test-realm";
pub const BFF_CLIENT_ID: &str = "webapp";
pub const BFF_CLIENT_SECRET: &str = "webapp-secret";
/// Registered on the `webapp` client. Nothing ever listens on it: the login agent
/// refuses redirects and reads the `Location` header instead.
pub const BFF_REDIRECT_URL: &str = "http://localhost:9999/callback";
pub const BFF_USERNAME: &str = "oidcuser";
pub const BFF_PASSWORD: &str = "oidcpass";
pub const BFF_USER_ID: &str = "22222222-2222-2222-2222-222222222222";
pub const BFF_EMAIL: &str = "oidcuser@example.com";
pub const BFF_GIVEN_NAME: &str = "Oidc";
pub const BFF_FAMILY_NAME: &str = "User";

/// Realms imported on container startup. Keycloak imports every file it finds in
/// the import directory, so both live in the same container.
const CREDENTIALS_REALM_EXPORT: &str = include_str!("../assets/realm-export.json");
const BFF_REALM_EXPORT: &str = include_str!("../assets/oidc-realm-export.json");

pub struct KeycloakFixture {
    #[allow(dead_code)]
    container: ContainerAsync<GenericImage>,
    pub base_url: String,
}

impl TestSuite for KeycloakFixture {
    async fn start() -> Self {
        Self::start_container().await
    }

    /// Both suites share their authenticator: building one re-fetches the realm's
    /// JWKS, and the login state they cache is keyed by a random CSRF value, so
    /// parallel trials cannot collide.
    fn trials(self: Arc<Self>, rt: Arc<Runtime>) -> Vec<Trial> {
        let credentials = Arc::new(rt.block_on(self.credentials_authenticator()));
        let bff = Arc::new(rt.block_on(self.bff_authenticator()));

        let mut trials = super::authenticator::suite::trials_shared(
            Arc::clone(&rt),
            credentials,
            Arc::clone(&self),
        );
        trials.extend(super::oidc::suite::trials_shared(rt, bff, self));
        trials
    }
}

impl KeycloakFixture {
    async fn start_container() -> Self {
        let container = GenericImage::new(KEYCLOAK_IMAGE, KEYCLOAK_TAG)
            .with_exposed_port(Tcp(KEYCLOAK_PORT))
            .with_wait_for(WaitFor::message_on_stdout("started in"))
            .with_cmd(["start-dev", "--import-realm"])
            .with_env_var("KC_BOOTSTRAP_ADMIN_USERNAME", "admin")
            .with_env_var("KC_BOOTSTRAP_ADMIN_PASSWORD", "admin")
            .with_copy_to(
                "/opt/keycloak/data/import/realm-export.json",
                CREDENTIALS_REALM_EXPORT.as_bytes().to_vec(),
            )
            .with_copy_to(
                "/opt/keycloak/data/import/oidc-realm-export.json",
                BFF_REALM_EXPORT.as_bytes().to_vec(),
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

    /// Realm base URL; every provider endpoint is derived from it.
    pub fn issuer_url(&self, realm: &str) -> String {
        format!("{}/realms/{realm}", self.base_url)
    }

    /// A backend for the credentials realm. Its OAuth half is wired but never
    /// exercised: that realm declares no confidential client.
    pub async fn credentials_authenticator(&self) -> Keycloak {
        self.authenticator(CREDENTIALS_REALM, CREDENTIALS_CLIENT_ID, "")
            .await
    }

    /// A backend for the Backend-for-Frontend realm.
    pub async fn bff_authenticator(&self) -> Keycloak {
        self.authenticator(BFF_REALM, BFF_CLIENT_ID, BFF_CLIENT_SECRET)
            .await
    }

    /// Builds a real backend pointed at this container.
    ///
    /// The cache is a working in-memory one because the login flow round-trips
    /// its PKCE verifier and CSRF state through it; the database only has to
    /// report "not found" for the API-key path.
    async fn authenticator(
        &self,
        realm: &str,
        client_id: &str,
        client_secret: &str,
    ) -> Keycloak {
        let cache: Arc<RwLock<dyn Cache>> =
            Arc::new(RwLock::new(HashMapCache::default()));
        let database: Arc<RwLock<dyn Database>> =
            Arc::new(RwLock::new(MockDatabase::default()));

        Keycloak::try_new(
            &self.config(realm, client_id, client_secret),
            cache,
            database,
        )
        .await
        .expect("failed to build the keycloak authenticator")
    }

    fn config(&self, realm: &str, client_id: &str, client_secret: &str) -> Config {
        let mut config = config::testing::test_config();
        config.authenticator.issuer_url = self.issuer_url(realm);
        config.authenticator.audiences = vec![AUDIENCE.to_string()];
        config.authenticator.client_id = client_id.to_string();
        config.authenticator.client_secret = client_secret.to_string();
        config.authenticator.redirect_url = BFF_REDIRECT_URL.to_string();
        config
    }
}
