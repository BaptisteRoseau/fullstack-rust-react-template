//! httpOnly cookies carrying the auth token pair set by the OAuth Backend-for-Frontend.

use authenticator::AuthTokens;
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};

/// Name of the httpOnly cookie holding the access token.
pub(crate) const ACCESS_COOKIE: &str = "access_token";
/// Name of the httpOnly cookie holding the refresh token.
pub(crate) const REFRESH_COOKIE: &str = "refresh_token";

/// Builds an httpOnly session cookie carrying a token.
pub(super) fn token_cookie(
    name: &'static str,
    value: String,
    secure: bool,
) -> Cookie<'static> {
    Cookie::build((name, value))
        .http_only(true)
        .same_site(SameSite::Lax)
        .path("/")
        .secure(secure)
        .build()
}

pub(super) fn set_token_cookies(
    jar: CookieJar,
    tokens: &AuthTokens,
    secure: bool,
) -> CookieJar {
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

pub(super) fn clear_token_cookies(jar: CookieJar) -> CookieJar {
    jar.remove(Cookie::build(ACCESS_COOKIE).path("/").build())
        .remove(Cookie::build(REFRESH_COOKIE).path("/").build())
}
