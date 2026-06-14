use crate::extractors::OptionalUser;
use crate::models::UserToken;
use crate::{AppState, error::ApiError, extractors::error::ExtractorError};
use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};

pub struct RequiredUser(UserToken);

impl RequiredUser {
    pub fn inner(self) -> UserToken {
        self.0
    }
}

impl From<RequiredUser> for UserToken {
    fn from(value: RequiredUser) -> Self {
        value.0
    }
}

impl From<UserToken> for RequiredUser {
    fn from(value: UserToken) -> Self {
        Self(value)
    }
}

impl<S> FromRequestParts<S> for RequiredUser
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = ApiError;

    /// Extract the user if possible. If not, directly return an error without getting to
    /// the handler.
    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        match OptionalUser::from_request_parts(parts, state).await {
            Ok(opt_user) => match opt_user.inner() {
                Some(user) => Ok(user.into()),
                None => Err(ApiError::from(ExtractorError::NotLoggedIn)),
            },
            Err(e) => Err(e),
        }
    }
}
