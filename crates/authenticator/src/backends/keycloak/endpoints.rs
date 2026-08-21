//! Keycloak's well-known endpoints, derived from the realm's issuer URL — the
//! single place where Keycloak's URL shape is encoded, so the JWKS URL and the
//! OAuth endpoints can never drift onto different realms.

const AUTH_PATH: &str = "/protocol/openid-connect/auth";
const REGISTRATIONS_PATH: &str = "/protocol/openid-connect/registrations";
const TOKEN_PATH: &str = "/protocol/openid-connect/token";
const LOGOUT_PATH: &str = "/protocol/openid-connect/logout";
const USERINFO_PATH: &str = "/protocol/openid-connect/userinfo";
const JWKS_PATH: &str = "/protocol/openid-connect/certs";

/// Keycloak's well-known endpoints, all derived from the realm base URL.
pub(super) struct Endpoints {
    pub(super) authorize: String,
    pub(super) registrations: String,
    pub(super) token: String,
    pub(super) logout: String,
    pub(super) userinfo: String,
    pub(super) jwks: String,
}

impl Endpoints {
    /// Derives every endpoint from the realm base URL, e.g.
    /// `http://localhost:8090/realms/app`. A trailing slash is trimmed first so
    /// the derived paths never end up with a doubled `/`.
    pub(super) fn from_issuer(issuer_url: &str) -> Self {
        let issuer = issuer_url.trim_end_matches('/');
        Self {
            authorize: format!("{issuer}{AUTH_PATH}"),
            registrations: format!("{issuer}{REGISTRATIONS_PATH}"),
            token: format!("{issuer}{TOKEN_PATH}"),
            logout: format!("{issuer}{LOGOUT_PATH}"),
            userinfo: format!("{issuer}{USERINFO_PATH}"),
            jwks: format!("{issuer}{JWKS_PATH}"),
        }
    }
}

test_utils::tests_file!("_tests/test_endpoints.rs");
