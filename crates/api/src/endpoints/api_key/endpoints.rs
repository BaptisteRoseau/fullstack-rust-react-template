use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use tokio::sync::RwLock;
use uuid::Uuid;

use super::models::{CreateApiKeyRequest, CreateApiKeyResponse, GetApiKeyResponse};
use crate::{
    app_state::AppState,
    error::{ApiError, ApiErrorResponse},
    models::UserToken,
};

fn parse_permissions(raw: &[String]) -> Vec<rbac::Permissions> {
    raw.iter()
        .filter_map(|s| serde_json::from_value(serde_json::Value::String(s.clone())).ok())
        .collect()
}

/// Create a new API key. Returns the raw key once — it cannot be retrieved again.
#[axum_macros::debug_handler(state = AppState)]
#[utoipa::path(
    post,
    path = "/api-key",
    request_body = CreateApiKeyRequest,
    responses(
        (status = CREATED, body = CreateApiKeyResponse, description = "API key created."),
        (status = UNAUTHORIZED, body = ApiErrorResponse, description = "Not authenticated."),
    ),
)]
pub(crate) async fn create_api_key(
    user: UserToken,
    State(db): State<Arc<RwLock<dyn database::Database>>>,
    Json(body): Json<CreateApiKeyRequest>,
) -> Result<(StatusCode, Json<CreateApiKeyResponse>), ApiError> {
    let permissions = parse_permissions(&body.permissions);
    let mut db = db.write().await;
    let (raw_key, api_key) =
        app_core::api_key::create_api_key(&mut *db, user.id, body.name, permissions)
            .await
            .map_err(|e| ApiError::Unexpected(anyhow::anyhow!("{e}")))?;

    Ok((
        StatusCode::CREATED,
        Json(CreateApiKeyResponse::new(raw_key, api_key)),
    ))
}

/// Get API key metadata by ID. Returns 404 if not owned by the caller.
#[axum_macros::debug_handler(state = AppState)]
#[utoipa::path(
    get,
    path = "/api-key/{id}",
    params(("id" = Uuid, Path, description = "API key ID")),
    responses(
        (status = OK, body = GetApiKeyResponse, description = "API key metadata."),
        (status = NOT_FOUND, description = "Not found or not owned by caller."),
        (status = UNAUTHORIZED, body = ApiErrorResponse, description = "Not authenticated."),
    ),
)]
pub(crate) async fn get_api_key(
    user: UserToken,
    State(db): State<Arc<RwLock<dyn database::Database>>>,
    Path(id): Path<Uuid>,
) -> Result<Json<GetApiKeyResponse>, ApiError> {
    let db = db.read().await;
    let db_key = db
        .read_api_key_by_id(id)
        .await
        .map_err(|_| ApiError::NotFound(id.to_string()))?;

    if db_key.owner() != user.id {
        return Err(ApiError::NotFound(id.to_string()));
    }

    let api_key = app_core::models::api_key_from_db(db_key)
        .map_err(|e| ApiError::Unexpected(anyhow::anyhow!("failed to deserialize permissions: {e}")))?;

    Ok(Json(GetApiKeyResponse::from(api_key)))
}

/// Delete an API key. Returns 404 if not owned by the caller.
#[axum_macros::debug_handler(state = AppState)]
#[utoipa::path(
    delete,
    path = "/api-key/{id}",
    params(("id" = Uuid, Path, description = "API key ID")),
    responses(
        (status = NO_CONTENT, description = "API key deleted."),
        (status = NOT_FOUND, description = "Not found or not owned by caller."),
        (status = UNAUTHORIZED, body = ApiErrorResponse, description = "Not authenticated."),
    ),
)]
pub(crate) async fn delete_api_key(
    user: UserToken,
    State(db): State<Arc<RwLock<dyn database::Database>>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    let db_key = {
        let db_read = db.read().await;
        db_read
            .read_api_key_by_id(id)
            .await
            .map_err(|_| ApiError::NotFound(id.to_string()))?
    };

    if db_key.owner() != user.id {
        return Err(ApiError::NotFound(id.to_string()));
    }

    db.write()
        .await
        .delete_api_key(id)
        .await
        .map_err(|e| ApiError::Unexpected(anyhow::anyhow!("{e}")))?;

    Ok(StatusCode::NO_CONTENT)
}
