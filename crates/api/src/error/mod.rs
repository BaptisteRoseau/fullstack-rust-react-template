#[allow(clippy::module_inception)]
mod error;
mod response;

pub(crate) use error::ApiError;
pub(crate) use response::ApiErrorResponse;
