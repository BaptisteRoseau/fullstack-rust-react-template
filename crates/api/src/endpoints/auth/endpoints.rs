//! OAuth Backend-for-Frontend endpoints.
//!
//! These are thin HTTP handlers: they call into [`OidcClient`] (which owns the OAuth
//! logic), translate the resulting tokens into httpOnly cookies, and issue the browser
//! redirects. The OAuth dance itself lives in the `authenticator` crate.

use std::sync::Arc;

use authenticator::{LoginScreen, OidcClient, OidcTokens};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{Json, Redirect},
};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use tokio::sync::RwLock;
use uuid::Uuid;

use super::models::{GetCallbackParams, GetLoginParams, GetMeResponse};
use crate::{
    error::{ApiError, ApiErrorResponse},
    extractors::error::ExtractorError,
    models::UserToken,
};

crate::endpoints::macros::declare_tag!(
    "Authentication",
    "Login, registration, token refresh and logout via OIDC."
);

/// Start the login flow by redirecting the browser to the OIDC provider.
#[utoipa::path(
    get,
    path = "/auth/login",
    tag = TAG,
    params(GetLoginParams),
    responses(
        (status = SEE_OTHER, description = "Redirect to the login page."),
    ),
)]
pub(crate) async fn login(
    State(oauth): State<Arc<OidcClient>>,
    Query(params): Query<GetLoginParams>,
) -> Result<Redirect, ApiError> {
    let url = oauth
        .authorize_url(LoginScreen::Login, params.redirect)
        .await?;
    Ok(Redirect::to(&url))
}

/// Start the registration flow by redirecting the browser to the OIDC provider.
#[utoipa::path(
    get,
    path = "/auth/register",
    tag = TAG,
    params(GetLoginParams),
    responses(
        (status = SEE_OTHER, description = "Redirect to the login or registration page."),
    ),
)]
pub(crate) async fn register(
    State(oauth): State<Arc<OidcClient>>,
    Query(params): Query<GetLoginParams>,
) -> Result<Redirect, ApiError> {
    let url = oauth
        .authorize_url(LoginScreen::Register, params.redirect)
        .await?;
    Ok(Redirect::to(&url))
}

/// OAuth callback: exchange the authorization code, set cookies, return to the app.
#[utoipa::path(
    get,
    path = "/auth/callback",
    tag = TAG,
    params(GetCallbackParams),
    responses(
        (status = SEE_OTHER, description = "Tokens stored in httpOnly cookies; redirect to the frontend."),
    ),
)]
pub(crate) async fn callback(
    State(oauth): State<Arc<OidcClient>>,
    State(database): State<Arc<RwLock<dyn database::Database>>>,
    jar: CookieJar,
    Query(params): Query<GetCallbackParams>,
) -> Result<(CookieJar, Redirect), ApiError> {
    let (Some(code), Some(state)) = (params.code, params.state) else {
        // No code (e.g. the user cancelled): bounce back to the frontend.
        return Ok((jar, Redirect::to(oauth.frontend_url())));
    };

    let (tokens, redirect) = oauth.exchange_code(code, state).await?;

    let info = oauth.userinfo(&tokens.access_token).await?;
    register_user(&info, &database).await?;

    let jar = set_token_cookies(jar, &tokens, oauth.cookie_secure());
    let target = frontend_target(oauth.frontend_url(), redirect.as_deref());
    Ok((jar, Redirect::to(&target)))
}

/// Silently refresh the access token using the refresh-token cookie.
#[utoipa::path(
    post,
    path = "/auth/refresh",
    tag = TAG,
    responses(
        (status = OK, description = "Access token refreshed; cookies updated."),
        (status = UNAUTHORIZED, description = "No valid refresh token; the user must log in again."),
    ),
)]
pub(crate) async fn refresh(
    State(oauth): State<Arc<OidcClient>>,
    jar: CookieJar,
) -> Result<(CookieJar, StatusCode), ApiError> {
    let Some(refresh_token) = jar.get(REFRESH_COOKIE).map(|c| c.value().to_string())
    else {
        return Err(ApiError::Unauthorized);
    };

    match oauth.refresh(refresh_token).await {
        Ok(tokens) => Ok((
            set_token_cookies(jar, &tokens, oauth.cookie_secure()),
            StatusCode::OK,
        )),
        Err(_) => Err(ApiError::Unauthorized),
    }
}

/// Revoke the session at the OIDC provider and clear the auth cookies.
#[utoipa::path(
    post,
    path = "/auth/logout",
    tag = TAG,
    responses(
        (status = NO_CONTENT, description = "Session revoked and cookies cleared."),
    ),
)]
pub(crate) async fn logout(
    State(oauth): State<Arc<OidcClient>>,
    jar: CookieJar,
) -> Result<(CookieJar, StatusCode), ApiError> {
    if let Some(refresh_token) = jar.get(REFRESH_COOKIE).map(|c| c.value().to_string()) {
        let _ = oauth.logout(refresh_token).await;
    }
    Ok((clear_token_cookies(jar), StatusCode::NO_CONTENT))
}

/// Return the current user's profile from the OIDC provider's userinfo endpoint.
#[utoipa::path(
    get,
    path = "/auth/me",
    tag = TAG,
    responses(
        (status = OK, body = GetMeResponse, description = "The authenticated user's profile."),
        (status = UNAUTHORIZED, body = ApiErrorResponse, description = "Not authenticated."),
    ),
)]
pub(crate) async fn me(
    _user: UserToken,
    State(oauth): State<Arc<OidcClient>>,
    jar: CookieJar,
) -> Result<Json<GetMeResponse>, ApiError> {
    let access_token = jar
        .get(ACCESS_COOKIE)
        .map(|c| c.value().to_string())
        .ok_or(ApiError::ExtractorError(ExtractorError::NotLoggedIn))?;

    let info = oauth.userinfo(&access_token).await?;
    Ok(Json(GetMeResponse::from_userinfo(&info)))
}

/* =======================================================================================
 * IMPLEMENTATION DETAILS
 * ===================================================================================== */

const ACCESS_COOKIE: &str = "access_token";
const REFRESH_COOKIE: &str = "refresh_token";

/// Creates or syncs the local user row from Keycloak's userinfo claims.
async fn register_user(
    info: &serde_json::Value,
    database: &Arc<RwLock<dyn database::Database>>,
) -> Result<(), ApiError> {
    let claim = |key: &str| {
        info.get(key)
            .and_then(serde_json::Value::as_str)
            .unwrap_or_default()
            .to_string()
    };

    let id = Uuid::parse_str(&claim("sub"))
        .map_err(|e| ApiError::Unexpected(anyhow::anyhow!("invalid sub claim: {e}")))?;

    let mut db = database.write().await;
    app_core::user::register(
        &mut *db,
        id,
        claim("preferred_username"),
        claim("given_name"),
        claim("family_name"),
        claim("email"),
    )
    .await?;

    Ok(())
}

/// Builds an httpOnly session cookie carrying a token.
fn token_cookie(name: &'static str, value: String, secure: bool) -> Cookie<'static> {
    Cookie::build((name, value))
        .http_only(true)
        .same_site(SameSite::Lax)
        .path("/")
        .secure(secure)
        .build()
}

fn set_token_cookies(jar: CookieJar, tokens: &OidcTokens, secure: bool) -> CookieJar {
    let mut jar = jar.add(token_cookie(
        ACCESS_COOKIE,
        tokens.access_token.clone(),
        secure,
    ));
    if let Some(refresh_token) = &tokens.refresh_token {
        jar = jar.add(token_cookie(REFRESH_COOKIE, refresh_token.clone(), secure));
    }
    jar
}

fn clear_token_cookies(jar: CookieJar) -> CookieJar {
    jar.remove(Cookie::build(ACCESS_COOKIE).path("/").build())
        .remove(Cookie::build(REFRESH_COOKIE).path("/").build())
}

/// Resolves the post-login redirect against the frontend origin. Only same-origin
/// paths (starting with `/`) are honored to avoid open redirects.
fn frontend_target(frontend_url: &str, redirect: Option<&str>) -> String {
    match redirect {
        Some(path) if path.starts_with('/') => {
            format!("{}{}", frontend_url.trim_end_matches('/'), path)
        }
        _ => frontend_url.to_string(),
    }
}
