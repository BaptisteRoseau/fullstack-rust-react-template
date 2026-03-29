use super::models::GetUserResponse;
use crate::error::ApiErrorResponse;
use crate::{AppState, error::ApiError};
use axum::extract::{Path, State};
use axum::response::Json;
use uuid::Uuid;

use crate::extractors::OptionalUser;

/// Get the information of a user.
#[axum_macros::debug_handler]
#[utoipa::path(
    get,
    path = "/user/{uuid}",
    responses(
        (status = OK, body = GetUserResponse, description = "The user information."),
        (status = NOT_FOUND, body = ApiErrorResponse, description = "The user does not exist."),
    ),
)]
pub(crate) async fn get_user(
    _uuid: Path<Uuid>,
    opt_user: OptionalUser,
    State(_state): State<AppState>,
) -> Result<Json<GetUserResponse>, ApiError> {
    match opt_user.inner() {
        Some(user) => Ok(GetUserResponse::from(user.name()).into()),
        None => Ok(GetUserResponse::from("Nothing".to_string()).into()),
    }
}
