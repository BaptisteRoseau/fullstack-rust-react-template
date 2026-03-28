use std::sync::{Arc, RwLock};

use super::models::GetUserResponse;
use crate::{error::ApiError};
use axum::extract::{Path, State};
use database::Database;
use uuid::Uuid;

use crate::extractors::OptionalUser;

/// Get the information of a user.
#[utoipa::path(
    get,
    path = "/user/:uuid",
    responses(
        (status = OK, description = "The user information."),
        (status = NOT_FOUND, description = "The user does not exist."),
    ),
)]
pub(crate) async fn get_user(
    _uuid: Path<Uuid>,
    opt_user: OptionalUser,
    State(_database): State<Arc<RwLock<dyn Database>>>,
) -> Result<GetUserResponse, ApiError> {
    match opt_user.inner() {
        Some(user) => Ok(GetUserResponse::from(user.name())),
        None => Ok(GetUserResponse::from("Nothing".to_string())),
    }
}
