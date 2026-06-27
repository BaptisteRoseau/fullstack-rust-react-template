use super::models::GetUserResponse;
use crate::error::ApiErrorResponse;
use crate::models::UserToken;
use crate::{AppState, error::ApiError};
use axum::extract::{Path, State};
use axum::response::Json;
use uuid::Uuid;

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
    opt_user: Option<UserToken>,
    State(_state): State<AppState>,
) -> Result<Json<GetUserResponse>, ApiError> {
    match opt_user {
        Some(user) => Ok(GetUserResponse::from(user.id.to_string()).into()),
        None => Ok(GetUserResponse::from("Nothing".to_string()).into()),
    }
}
