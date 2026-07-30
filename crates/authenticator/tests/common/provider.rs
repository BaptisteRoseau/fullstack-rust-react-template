//! The provider-side actor the trait suite drives.
//!
//! Two things in the suite cannot be expressed through the `Authenticator` trait
//! itself: acting as the end user in front of the provider's login page, and
//! minting a credential to hand back to `validate`. Both are provider-specific,
//! so they sit behind [`ProviderAgent`] and the trait tests stay backend-agnostic.

use async_trait::async_trait;
use reqwest::header::{COOKIE, SET_COOKIE};
use serde_json::Value;

use super::containers::{
    BFF_PASSWORD, BFF_USERNAME, CREDENTIALS_CLIENT_ID, CREDENTIALS_PASSWORD,
    CREDENTIALS_REALM, CREDENTIALS_USERNAME, KeycloakFixture,
};

/// The query parameters the provider hands back to the redirect URI.
pub struct CallbackParams {
    pub code: String,
    pub state: String,
}

#[async_trait]
pub trait ProviderAgent {
    /// Acts as the end user at the provider's login page and returns the `code`
    /// and `state` the provider redirects back to the callback URL with.
    async fn login(&self, authorize_url: &str) -> CallbackParams;

    /// A freshly issued access token for the credentials realm.
    async fn issue_token(&self) -> String;
}

#[async_trait]
impl ProviderAgent for KeycloakFixture {
    async fn login(&self, authorize_url: &str) -> CallbackParams {
        // Refuse redirects: the registered callback URL has no server behind it,
        // so the authorization code is read straight off the `Location` header.
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
        // Keycloak marks its login cookies `Secure`, which makes an automatic
        // cookie store drop them over plain HTTP. Echo the values back by hand.
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

/// Joins every `Set-Cookie` value into a `Cookie` header, keeping only the
/// `name=value` pair and discarding the attributes the server set.
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

/// Extracts the `action` URL of Keycloak's login form. The image tag is pinned,
/// so a Keycloak upgrade that reshapes this form surfaces right here.
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

/// Reads a query parameter out of a URL. The provider only sends URL-safe values
/// here (`code` and `state`), so no percent-decoding is needed.
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

/// Trims a response body down to something readable in a panic message.
fn snippet(body: &str) -> String {
    body.chars().take(300).collect()
}
