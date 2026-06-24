use axum::{
    extract::{Query, State},
    http::{HeaderValue, StatusCode, header},
    response::{IntoResponse, Json, Response},
};
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::{Cookie, SameSite};
use config::Config;
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
use rand::RngExt;
use std::sync::Arc;

use super::models::{GetMeResponse, KeycloakTokenResponse, LoginParams, UserRole};
use crate::{
    app_state::AppState,
    error::{ApiError, ApiErrorResponse},
    extractors::RequiredUser,
};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Build an HttpOnly auth cookie with common attributes.
fn auth_cookie(name: &str, value: String, max_age_secs: i64, secure: bool) -> Cookie<'static> {
    let mut c = Cookie::new(name.to_owned(), value);
    c.set_http_only(true);
    c.set_same_site(SameSite::Lax);
    c.set_path("/");
    c.set_max_age(time::Duration::seconds(max_age_secs));
    c.set_secure(secure);
    c
}

/// Build an expired cookie that clears a given name.
fn expired_cookie(name: &'static str) -> Cookie<'static> {
    let mut c = Cookie::new(name, "");
    c.set_http_only(true);
    c.set_same_site(SameSite::Lax);
    c.set_path("/");
    c.set_max_age(time::Duration::seconds(0));
    c
}

/// 302 redirect response.
fn redirect(location: &str) -> Response {
    (
        StatusCode::FOUND,
        [(
            header::LOCATION,
            HeaderValue::from_str(location).unwrap_or_else(|_| HeaderValue::from_static("/")),
        )],
    )
        .into_response()
}

/// Encode the `oauth_state` cookie value: `<random_hex>:<post_login_redirect>`.
fn encode_state_cookie(state_token: &str, post_login_redirect: &str) -> String {
    format!("{state_token}:{post_login_redirect}")
}

/// Decode the `oauth_state` cookie value, returning `(state_token, post_login_redirect)`.
fn decode_state_cookie(value: &str) -> Option<(String, String)> {
    let mut parts = value.splitn(2, ':');
    let state_token = parts.next()?.to_string();
    let redirect = parts.next()?.to_string();
    Some((state_token, redirect))
}

/// Generate a cryptographically random hex state token.
fn random_state() -> String {
    let bytes: [u8; 32] = rand::rng().random();
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

// ---------------------------------------------------------------------------
// Handlers — browser-facing (redirect flows)
// ---------------------------------------------------------------------------

/// Initiate the OIDC authorization-code flow. Redirects the browser to Keycloak.
#[axum_macros::debug_handler(state = AppState)]
#[utoipa::path(
    get,
    path = "/auth/login",
    params(LoginParams),
    responses(
        (status = 302, description = "Redirect to Keycloak authorization endpoint."),
    ),
)]
pub(crate) async fn login(
    State(config): State<Arc<Config>>,
    jar: CookieJar,
    Query(params): Query<LoginParams>,
) -> impl IntoResponse {
    let oidc = &config.oidc;
    let state_token = random_state();
    let post_login_redirect = params
        .redirect
        .unwrap_or_else(|| oidc.post_login_url.clone());

    let state_cookie_value = encode_state_cookie(&state_token, &post_login_redirect);
    let state_cookie = auth_cookie("oauth_state", state_cookie_value, 600, oidc.cookie_secure);

    let client_id_enc = utf8_percent_encode(&oidc.client_id, NON_ALPHANUMERIC).to_string();
    let redirect_uri_enc = utf8_percent_encode(&oidc.redirect_url, NON_ALPHANUMERIC).to_string();
    let state_enc = utf8_percent_encode(&state_token, NON_ALPHANUMERIC).to_string();

    let authorize_url = format!(
        "{}?response_type=code&client_id={}&redirect_uri={}&scope=openid%20email%20profile&state={}",
        oidc.authorize_url(),
        client_id_enc,
        redirect_uri_enc,
        state_enc,
    );

    (jar.add(state_cookie), redirect(&authorize_url)).into_response()
}

/// Query parameters for the OIDC callback endpoint.
#[derive(serde::Deserialize, utoipa::IntoParams)]
pub(crate) struct CallbackParams {
    pub code: String,
    pub state: String,
}

/// Handle the OIDC callback: verify state, exchange code for tokens, set cookies.
#[axum_macros::debug_handler(state = AppState)]
#[utoipa::path(
    get,
    path = "/auth/callback",
    params(CallbackParams),
    responses(
        (status = 302, description = "Redirect to the frontend post-login URL."),
        (status = 400, description = "Missing or mismatched state parameter."),
        (status = 502, description = "Keycloak token exchange failed."),
    ),
)]
pub(crate) async fn callback(
    State(config): State<Arc<Config>>,
    State(http): State<Arc<reqwest::Client>>,
    jar: CookieJar,
    Query(params): Query<CallbackParams>,
) -> impl IntoResponse {
    let oidc = &config.oidc;

    // Verify state to prevent CSRF.
    let state_cookie = match jar.get("oauth_state") {
        Some(c) => c.value().to_owned(),
        None => {
            return (StatusCode::BAD_REQUEST, "Missing oauth_state cookie").into_response();
        }
    };
    let (expected_state, post_login_redirect) = match decode_state_cookie(&state_cookie) {
        Some(v) => v,
        None => {
            return (StatusCode::BAD_REQUEST, "Malformed oauth_state cookie").into_response();
        }
    };
    if params.state != expected_state {
        return (StatusCode::BAD_REQUEST, "State mismatch — possible CSRF").into_response();
    }

    // Exchange code for tokens.
    let token_response = match http
        .post(oidc.token_url())
        .form(&[
            ("grant_type", "authorization_code"),
            ("code", params.code.as_str()),
            ("redirect_uri", oidc.redirect_url.as_str()),
            ("client_id", oidc.client_id.as_str()),
            ("client_secret", oidc.client_secret.as_str()),
        ])
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => {
            tracing::error!("Token exchange request failed: {e}");
            return StatusCode::BAD_GATEWAY.into_response();
        }
    };

    if !token_response.status().is_success() {
        let status = token_response.status();
        tracing::error!("Keycloak token endpoint returned {status}");
        return StatusCode::BAD_GATEWAY.into_response();
    }

    let tokens: KeycloakTokenResponse = match token_response.json().await {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("Failed to parse token response: {e}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let access_cookie = auth_cookie(
        "access_token",
        tokens.access_token,
        tokens.expires_in as i64,
        oidc.cookie_secure,
    );
    let refresh_cookie = auth_cookie(
        "refresh_token",
        tokens.refresh_token,
        tokens.refresh_expires_in as i64,
        oidc.cookie_secure,
    );

    (
        jar.remove(expired_cookie("oauth_state"))
            .add(access_cookie)
            .add(refresh_cookie),
        redirect(&post_login_redirect),
    )
        .into_response()
}

/// Clear the user's auth cookies and best-effort notify Keycloak's end-session endpoint.
#[axum_macros::debug_handler(state = AppState)]
#[utoipa::path(
    post,
    path = "/auth/logout",
    responses(
        (status = NO_CONTENT, description = "Logged out successfully."),
    ),
)]
pub(crate) async fn logout(
    State(config): State<Arc<Config>>,
    State(http): State<Arc<reqwest::Client>>,
    jar: CookieJar,
) -> impl IntoResponse {
    let oidc = &config.oidc;

    // Best-effort end-session call to Keycloak with the refresh token.
    if let Some(refresh_cookie) = jar.get("refresh_token") {
        let _ = http
            .post(oidc.end_session_url())
            .form(&[
                ("client_id", oidc.client_id.as_str()),
                ("client_secret", oidc.client_secret.as_str()),
                ("refresh_token", refresh_cookie.value()),
            ])
            .send()
            .await;
    }

    (
        jar.remove(expired_cookie("access_token"))
            .remove(expired_cookie("refresh_token")),
        StatusCode::NO_CONTENT,
    )
        .into_response()
}

// ---------------------------------------------------------------------------
// Handlers — JSON endpoints (OpenAPI-documented)
// ---------------------------------------------------------------------------

/// Return identity information about the currently authenticated user.
#[axum_macros::debug_handler(state = AppState)]
#[utoipa::path(
    get,
    path = "/auth/me",
    responses(
        (status = OK, body = GetMeResponse, description = "The authenticated user's identity."),
        (status = UNAUTHORIZED, body = ApiErrorResponse, description = "Not authenticated or token invalid."),
    ),
)]
pub(crate) async fn get_me(
    user: RequiredUser,
) -> Result<Json<GetMeResponse>, ApiError> {
    let token = user.inner();
    let role = if token.roles.iter().any(|r| r == "ADMIN") {
        UserRole::Admin
    } else {
        UserRole::User
    };
    Ok(Json(GetMeResponse {
        id: token.id,
        email: token.email,
        role,
    }))
}

/// Refresh the access token using the refresh-token cookie.
///
/// On success the `access_token` (and `refresh_token` if rotated) cookies are updated.
#[axum_macros::debug_handler(state = AppState)]
#[utoipa::path(
    post,
    path = "/auth/refresh",
    responses(
        (status = NO_CONTENT, description = "Tokens refreshed successfully."),
        (status = UNAUTHORIZED, body = ApiErrorResponse, description = "Refresh token missing or rejected by Keycloak."),
    ),
)]
pub(crate) async fn refresh_token(
    State(config): State<Arc<Config>>,
    State(http): State<Arc<reqwest::Client>>,
    jar: CookieJar,
) -> Result<impl IntoResponse, ApiError> {
    let oidc = &config.oidc;

    let refresh_value = jar
        .get("refresh_token")
        .map(|c| c.value().to_owned())
        .ok_or_else(|| {
            ApiError::Unexpected(anyhow::anyhow!("No refresh_token cookie present"))
        })?;

    let response = http
        .post(oidc.token_url())
        .form(&[
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_value.as_str()),
            ("client_id", oidc.client_id.as_str()),
            ("client_secret", oidc.client_secret.as_str()),
        ])
        .send()
        .await
        .map_err(|e| ApiError::Unexpected(anyhow::anyhow!("Token refresh request failed: {e}")))?;

    if !response.status().is_success() {
        return Err(ApiError::Unexpected(anyhow::anyhow!(
            "Keycloak rejected refresh: {}",
            response.status()
        )));
    }

    let tokens: KeycloakTokenResponse = response
        .json()
        .await
        .map_err(|e| ApiError::Unexpected(anyhow::anyhow!("Failed to parse refresh response: {e}")))?;

    let access_cookie = auth_cookie(
        "access_token",
        tokens.access_token,
        tokens.expires_in as i64,
        oidc.cookie_secure,
    );
    let refresh_cookie = auth_cookie(
        "refresh_token",
        tokens.refresh_token,
        tokens.refresh_expires_in as i64,
        oidc.cookie_secure,
    );

    Ok((
        jar.add(access_cookie).add(refresh_cookie),
        StatusCode::NO_CONTENT,
    ))
}
