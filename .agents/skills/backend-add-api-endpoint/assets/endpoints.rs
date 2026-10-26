//! Handlers for this endpoint group.
//!
//! A handler extracts input, calls `app_core`, and serialises the result.
//! It contains no business logic and no access check: it passes the caller to `app_core`.

use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use tokio::sync::RwLock;
use uuid::Uuid;

use super::models::{GetWidgetResponse, PostWidgetRequest};
use crate::{
    app_state::AppState,
    error::{ApiError, ApiErrorResponse},
    models::UserToken,
};

/// Get a widget by its identifier.
#[axum_macros::debug_handler(state = AppState)]
#[utoipa::path(
    get,
    path = "/widget/{id}",
    params(("id" = Uuid, Path, description = "Widget ID")),
    responses(
        (status = OK, body = GetWidgetResponse, description = "The widget."),
        (status = NOT_FOUND, body = ApiErrorResponse, description = "No such widget."),
        (status = UNAUTHORIZED, body = ApiErrorResponse, description = "Not authenticated."),
    ),
)]
pub(crate) async fn get_widget(
    user: UserToken,
    State(db): State<Arc<RwLock<dyn database::Database>>>,
    Path(id): Path<Uuid>,
) -> Result<Json<GetWidgetResponse>, ApiError> {
    // `app_core` locks the database and runs the access check. It answers `NotFound` on
    // refusal, which `?` turns into a 404.
    let widget = app_core::widget::get_widget(&db, user.caller(), id).await?;

    Ok(Json(GetWidgetResponse::from(widget)))
}

/// Create a widget.
#[axum_macros::debug_handler(state = AppState)]
#[utoipa::path(
    post,
    path = "/widget",
    request_body = PostWidgetRequest,
    responses(
        (status = CREATED, body = GetWidgetResponse, description = "The created widget."),
        (status = UNAUTHORIZED, body = ApiErrorResponse, description = "Not authenticated."),
    ),
)]
pub(crate) async fn create_widget(
    user: UserToken,
    State(db): State<Arc<RwLock<dyn database::Database>>>,
    Json(body): Json<PostWidgetRequest>,
) -> Result<(StatusCode, Json<GetWidgetResponse>), ApiError> {
    let widget = app_core::widget::create_widget(&db, user.caller(), body.name)
        .await
        .map_err(|error| ApiError::Unexpected(anyhow::anyhow!("{error}")))?;

    Ok((StatusCode::CREATED, Json(GetWidgetResponse::from(widget))))
}
