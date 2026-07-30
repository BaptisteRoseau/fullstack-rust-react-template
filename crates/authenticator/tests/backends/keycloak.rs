//! Runs the `Authenticator` trait suites against the `Keycloak` backend.

use std::sync::Arc;

use async_trait::async_trait;
use reqwest::header::{COOKIE, SET_COOKIE};
use serde_json::Value;
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

#[path = "../trait_tests/mod.rs"]
mod trait_tests;

use trait_tests::provider::{CallbackParams, ProviderAgent};
use trait_tests::{BFF_CLIENT_ID, BFF_REALM, BFF_USERNAME, CREDENTIALS_REALM};

test_trait::test_trait_main!(KeycloakFixture);

struct KeycloakFixture {
    _container: ContainerAsync<GenericImage>,
    base_url: String,
}

impl TestSuite for KeycloakFixture {
    async fn start() -> Self {
        Self::start_container().await
    }

    fn trials(self: Arc<Self>, rt: Arc<Runtime>) -> Vec<Trial> {
        let credentials = Arc::new(rt.block_on(self.credentials_authenticator()));
        let bff = Arc::new(rt.block_on(self.bff_authenticator()));

        let mut trials = trait_tests::credentials::suite::trials_shared(
            Arc::clone(&rt),
            credentials,
            Arc::clone(&self),
        );
        trials.extend(trait_tests::oidc::suite::trials_shared(rt, bff, self));
        trials
    }
}

#[async_trait]
impl ProviderAgent for KeycloakFixture {
    async fn login(&self, authorize_url: &str) -> CallbackParams {
        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect("failed to build the login http client");

        let page = client
            .get(authorize_url)
            .send()
            .await
            .expect("the authorize request failed");
        let status = page.status();
        let cookies = collect_cookies(&page);
        let html = page.text().await.expect("the login page was not text");
        assert!(
            status.is_success(),
            "the authorize request should serve the login page, got={status} body={}",
            snippet(&html)
        );

        let response = client
            .post(login_form_action(&html))
            .header(COOKIE, cookies)
            .form(&[
                ("username", BFF_USERNAME),
                ("password", BFF_PASSWORD),
                ("credentialId", ""),
            ])
            .send()
            .await
            .expect("the login form submission failed");

        let status = response.status();
        let location = response
            .headers()
            .get(reqwest::header::LOCATION)
            .and_then(|value| value.to_str().ok())
            .map(str::to_string);
        let Some(location) = location else {
            let body = response.text().await.unwrap_or_default();
            panic!(
                "signing in should redirect to the callback URL, got={status} body={}",
                snippet(&body)
            );
        };

        CallbackParams {
            code: query_param(&location, "code"),
            state: query_param(&location, "state"),
        }
    }

    async fn issue_token(&self) -> String {
        let url = format!(
            "{}/protocol/openid-connect/token",
            self.issuer_url(CREDENTIALS_REALM)
        );

        let response = reqwest::Client::new()
            .post(&url)
            .form(&[
                ("grant_type", "password"),
                ("client_id", CREDENTIALS_CLIENT_ID),
                ("username", CREDENTIALS_USERNAME),
                ("password", CREDENTIALS_PASSWORD),
                ("scope", "openid"),
            ])
            .send()
            .await
            .expect("the token request failed");

        let status = response.status();
        let body: Value = response.json().await.unwrap_or_else(|e| {
            panic!("the token response was not JSON (status {status}): {e}")
        });

        body["access_token"]
            .as_str()
            .unwrap_or_else(|| {
                panic!("no access_token in the token response (status {status}): {body}")
            })
            .to_string()
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
            _container: container,
            base_url: format!("http://127.0.0.1:{port}"),
        }
    }

    fn issuer_url(&self, realm: &str) -> String {
        format!("{}/realms/{realm}", self.base_url)
    }

    async fn credentials_authenticator(&self) -> Keycloak {
        self.authenticator(CREDENTIALS_REALM, CREDENTIALS_CLIENT_ID, "")
            .await
    }

    async fn bff_authenticator(&self) -> Keycloak {
        self.authenticator(BFF_REALM, BFF_CLIENT_ID, BFF_CLIENT_SECRET)
            .await
    }

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

const KEYCLOAK_IMAGE: &str = "quay.io/keycloak/keycloak";
const KEYCLOAK_TAG: &str = "26.6.4";
const KEYCLOAK_PORT: u16 = 8080;

const AUDIENCE: &str = "backend";

const CREDENTIALS_CLIENT_ID: &str = "backend";
const CREDENTIALS_USERNAME: &str = "testuser";
const CREDENTIALS_PASSWORD: &str = "testpass";

const BFF_CLIENT_SECRET: &str = "webapp-secret";
const BFF_PASSWORD: &str = "oidcpass";
const BFF_REDIRECT_URL: &str = "http://localhost:9999/callback";

const CREDENTIALS_REALM_EXPORT: &str = include_str!("../assets/realm-export.json");
const BFF_REALM_EXPORT: &str = include_str!("../assets/oidc-realm-export.json");

fn collect_cookies(response: &reqwest::Response) -> String {
    response
        .headers()
        .get_all(SET_COOKIE)
        .iter()
        .filter_map(|value| value.to_str().ok())
        .filter_map(|cookie| cookie.split(';').next())
        .collect::<Vec<_>>()
        .join("; ")
}

fn login_form_action(html: &str) -> String {
    let form_tag = html
        .split('<')
        .find(|tag| tag.starts_with("form") && tag.contains("kc-form-login"))
        .unwrap_or_else(|| panic!("no login form in the page, body={}", snippet(html)));

    let action = form_tag
        .split_once("action=\"")
        .unwrap_or_else(|| panic!("the login form has no action, tag={form_tag}"))
        .1
        .split_once('"')
        .unwrap_or_else(|| {
            panic!("the login form action is unterminated, tag={form_tag}")
        })
        .0;

    action.replace("&amp;", "&")
}

fn query_param(url: &str, key: &str) -> String {
    url.split_once('?')
        .unwrap_or_else(|| panic!("no query string in url={url}"))
        .1
        .split('&')
        .filter_map(|pair| pair.split_once('='))
        .find(|(name, _)| *name == key)
        .unwrap_or_else(|| panic!("no {key:?} parameter in url={url}"))
        .1
        .to_string()
}

fn snippet(body: &str) -> String {
    body.chars().take(300).collect()
}
