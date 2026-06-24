use crate::models::UserToken;
use crate::{AppState, error::ApiError};
use axum::{
    RequestPartsExt,
    extract::{FromRef, FromRequestParts},
    http::{HeaderMap, header, request::Parts},
};
use axum_extra::extract::CookieJar;
use tracing::debug;

pub struct OptionalUser(Option<UserToken>);

impl OptionalUser {
    pub fn inner(self) -> Option<UserToken> {
        self.0
    }
}

impl From<OptionalUser> for Option<UserToken> {
    fn from(value: OptionalUser) -> Self {
        value.0
    }
}

impl<S> FromRequestParts<S> for OptionalUser
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let headers = HeaderMap::from_request_parts(parts, state)
            .await
            .map_err(anyhow::Error::from)?;

        // Prefer the Authorization: Bearer header (API keys / programmatic clients).
        let token_opt = if let Some(auth_header) = headers.get(header::AUTHORIZATION) {
            let raw = auth_header.to_str().map_err(|e| anyhow::anyhow!(e))?;
            Some(raw.strip_prefix("Bearer ").unwrap_or(raw).to_owned())
        } else {
            // Fall back to the HttpOnly access_token cookie set by the BFF flow.
            let jar = CookieJar::from_request_parts(parts, state)
                .await
                .map_err(anyhow::Error::from)?;
            jar.get("access_token")
                .map(|c| c.value().to_owned())
        };

        let token = match token_opt {
            Some(t) => t,
            None => {
                debug!("Anonymous user");
                return Ok(OptionalUser(None));
            }
        };

        let app_state = parts.extract_with_state::<AppState, _>(state).await?;
        let user;
        {
            let authenticator = app_state.authenticator.read().await;
            user = authenticator.validate(&token).await?;
        }
        Ok(OptionalUser(Some(user.into())))
    }
}
