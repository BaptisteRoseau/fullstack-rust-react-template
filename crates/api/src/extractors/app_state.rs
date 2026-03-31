use crate::{AppState, error::ApiError};
use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};

// Required for `FromRequestParts` to compile.
// See: https://github.com/tokio-rs/axum/discussions/1732#discussioncomment-4878401
//
// Allows to use the AppState in extractors as follows:
//
// ```rs
// let app_state = parts.extract_with_state::<AppState, _>(state).await?;
// ```
impl<S> FromRequestParts<S> for AppState
where
    Self: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(
        _parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        Ok(Self::from_ref(state))
    }
}
