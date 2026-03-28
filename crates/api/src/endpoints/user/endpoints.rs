use super::models::GetUserResponse;
use crate::error::ApiError;
use axum::extract::Path;
use uuid::Uuid;

use crate::extractors::OptionalUser;

/// Get the information of a user.
#[utoipa::path(
    get,
    path = "/user/:uuid",
    responses(
        (status = OK, description = "The API is up and running."),
        (status = NOT_FOUND, description = "The user does not exist."),
    ),
)]
pub(crate) async fn get_user(
    // State(database): Arc<RwLock<dyn Database>>,
    OptionalUser(opt_user): OptionalUser,
    _uuid: Path<Uuid>,
) -> Result<GetUserResponse, ApiError> {
    todo!()
}
