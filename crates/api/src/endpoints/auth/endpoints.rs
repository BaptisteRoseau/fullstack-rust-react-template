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

use super::models::{GetCallbackParams, GetLoginParams, GetMeResponse};
use crate::{
    error::{ApiError, ApiErrorResponse},
    extractors::error::ExtractorError,
    models::UserToken,
};

/// Start the login (or registration) flow by redirecting the browser to Keycloak.
#[utoipa::path(
    get,
    path = "/auth/login",
    params(GetLoginParams),
    responses(
        (status = SEE_OTHER, description = "Redirect to the Keycloak login or registration page."),
    ),
)]
pub(crate) async fn login(
    State(oauth): State<Arc<OidcClient>>,
    Query(params): Query<GetLoginParams>,
) -> Result<Redirect, ApiError> {
    let screen = match params.screen.as_deref() {
        Some("register") => LoginScreen::Register,
        _ => LoginScreen::Login,
    };
    let url = oauth.authorize_url(screen, params.redirect).await?;
    Ok(Redirect::to(&url))
}

/// OAuth callback: exchange the authorization code, set cookies, return to the app.
#[utoipa::path(
    get,
    path = "/auth/callback",
    params(GetCallbackParams),
    responses(
        (status = SEE_OTHER, description = "Tokens stored in httpOnly cookies; redirect to the frontend."),
    ),
)]
pub(crate) async fn callback(
    State(oauth): State<Arc<OidcClient>>,
    jar: CookieJar,
    Query(params): Query<GetCallbackParams>,
) -> Result<(CookieJar, Redirect), ApiError> {
    let (Some(code), Some(state)) = (params.code, params.state) else {
        // No code (e.g. the user cancelled): bounce back to the frontend.
        return Ok((jar, Redirect::to(oauth.frontend_url())));
    };

    let (tokens, redirect) = oauth.exchange_code(code, state).await?;
    let jar = set_token_cookies(jar, &tokens, oauth.cookie_secure());
    let target = frontend_target(oauth.frontend_url(), redirect.as_deref());
    Ok((jar, Redirect::to(&target)))
}

/// Silently refresh the access token using the refresh-token cookie.
#[utoipa::path(
    post,
    path = "/auth/refresh",
    responses(
        (status = OK, description = "Access token refreshed; cookies updated."),
        (status = UNAUTHORIZED, description = "No valid refresh token; the user must log in again."),
    ),
)]
pub(crate) async fn refresh(
    State(oauth): State<Arc<OidcClient>>,
    jar: CookieJar,
) -> (CookieJar, StatusCode) {
    let Some(refresh_token) = jar.get(REFRESH_COOKIE).map(|c| c.value().to_string())
    else {
        return (jar, StatusCode::UNAUTHORIZED);
    };

    match oauth.refresh(refresh_token).await {
        Ok(tokens) => (
            set_token_cookies(jar, &tokens, oauth.cookie_secure()),
            StatusCode::OK,
        ),
        Err(_) => (clear_token_cookies(jar), StatusCode::UNAUTHORIZED),
    }
}

/// Revoke the session at Keycloak and clear the auth cookies.
#[utoipa::path(
    post,
    path = "/auth/logout",
    responses(
        (status = NO_CONTENT, description = "Session revoked and cookies cleared."),
    ),
)]
pub(crate) async fn logout(
    State(oauth): State<Arc<OidcClient>>,
    jar: CookieJar,
) -> (CookieJar, StatusCode) {
    if let Some(refresh_token) = jar.get(REFRESH_COOKIE).map(|c| c.value().to_string()) {
        let _ = oauth.logout(refresh_token).await;
    }
    (clear_token_cookies(jar), StatusCode::NO_CONTENT)
}

/// Return the current user's profile from Keycloak's userinfo endpoint.
#[utoipa::path(
    get,
    path = "/auth/me",
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
