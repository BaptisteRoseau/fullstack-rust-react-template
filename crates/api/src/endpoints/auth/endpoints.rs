//! OAuth Backend-for-Frontend endpoints.
//!
//! These are thin HTTP handlers: they call into the [`Authenticator`] trait (which owns
//! the OAuth logic), translate the resulting tokens into httpOnly cookies, and issue the
//! browser redirects. The OAuth dance itself lives in the `authenticator` crate.

use std::sync::Arc;

use authenticator::{Authenticator, LoginScreen, UserInfo};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{Json, Redirect},
};
use axum_extra::extract::cookie::CookieJar;
use config::Config;
use tokio::sync::RwLock;

use super::cookies::{
    ACCESS_COOKIE, REFRESH_COOKIE, clear_token_cookies, set_token_cookies,
};
use super::models::{GetCallbackParams, GetLoginParams, GetMeResponse};
use super::redirects::frontend_target;
use crate::{
    error::{ApiError, ApiErrorResponse},
    extractors::error::ExtractorError,
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
    State(authenticator): State<Arc<RwLock<dyn Authenticator>>>,
    Query(params): Query<GetLoginParams>,
) -> Result<Redirect, ApiError> {
    let url = authenticator
        .read()
        .await
        .authorize_url(LoginScreen::Login, params.redirect.as_deref())
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
    State(authenticator): State<Arc<RwLock<dyn Authenticator>>>,
    Query(params): Query<GetLoginParams>,
) -> Result<Redirect, ApiError> {
    let url = authenticator
        .read()
        .await
        .authorize_url(LoginScreen::Register, params.redirect.as_deref())
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
    State(authenticator): State<Arc<RwLock<dyn Authenticator>>>,
    State(database): State<Arc<RwLock<dyn database::Database>>>,
    State(config): State<Arc<Config>>,
    jar: CookieJar,
    Query(params): Query<GetCallbackParams>,
) -> Result<(CookieJar, Redirect), ApiError> {
    let frontend_url = &config.api.frontend_url;
    let (Some(code), Some(state)) = (params.code, params.state) else {
        // No code (e.g. the user cancelled): bounce back to the frontend.
        return Ok((jar, Redirect::to(frontend_url)));
    };

    let session = authenticator
        .read()
        .await
        .exchange_code(&code, &state)
        .await?;

    let info = authenticator
        .read()
        .await
        .userinfo(&session.tokens.access_token)
        .await?;
    register_user(&info, &database).await?;

    let jar = set_token_cookies(jar, &session.tokens, config.api.cookie_secure);
    let target = frontend_target(frontend_url, session.redirect.as_deref());
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
    State(authenticator): State<Arc<RwLock<dyn Authenticator>>>,
    State(config): State<Arc<Config>>,
    jar: CookieJar,
) -> Result<(CookieJar, StatusCode), ApiError> {
    let Some(refresh_token) = jar.get(REFRESH_COOKIE).map(|c| c.value().to_string())
    else {
        return Err(ApiError::Unauthorized);
    };

    match authenticator
        .read()
        .await
        .refresh_tokens(&refresh_token)
        .await
    {
        Ok(tokens) => Ok((
            set_token_cookies(jar, &tokens, config.api.cookie_secure),
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
    State(authenticator): State<Arc<RwLock<dyn Authenticator>>>,
    jar: CookieJar,
) -> Result<(CookieJar, StatusCode), ApiError> {
    if let Some(refresh_token) = jar.get(REFRESH_COOKIE).map(|c| c.value().to_string()) {
        let _ = authenticator.read().await.logout(&refresh_token).await;
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
    State(authenticator): State<Arc<RwLock<dyn Authenticator>>>,
    jar: CookieJar,
) -> Result<Json<GetMeResponse>, ApiError> {
    let access_token = jar
        .get(ACCESS_COOKIE)
        .map(|c| c.value().to_string())
        .ok_or(ApiError::ExtractorError(ExtractorError::NotLoggedIn))?;

    let info = authenticator.read().await.userinfo(&access_token).await?;
    Ok(Json(GetMeResponse::from_userinfo(&info)))
}

/* =======================================================================================
 * IMPLEMENTATION DETAILS
 * ===================================================================================== */

/// Creates or syncs the local user row from Keycloak's userinfo claims.
async fn register_user(
    info: &UserInfo,
    database: &Arc<RwLock<dyn database::Database>>,
) -> Result<(), ApiError> {
    let mut db = database.write().await;
    app_core::user::register(
        &mut *db,
        info.sub,
        info.preferred_username.clone(),
        info.given_name.clone(),
        info.family_name.clone(),
        info.email.clone(),
    )
    .await?;

    Ok(())
}
