use std::path::Path as FilePath;
use std::sync::Arc;

use axum::body::Bytes;
use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, header};
use axum::response::IntoResponse;
use storage::Storage;
use tokio::sync::RwLock;

use super::models::{PostUploadParams, PostUploadResponse};
use crate::error::ApiError;
use crate::extractors::OptionalUser;

/// Upload a file to storage.
#[utoipa::path(
    post,
    path = "/upload/{file}",
    request_body(content = String, content_type = "application/octet-stream", description = "The raw file content"),
    params(
        ("file" = String, Path, description = "The file path/name to store"),
        PostUploadParams,
    ),
    responses(
        (status = OK, body = PostUploadResponse, description = "File has been successfully uploaded."),
        (status = INTERNAL_SERVER_ERROR, description = "Storage error."),
    ),
)]
pub(crate) async fn upload(
    _user: OptionalUser,
    State(storage): State<Arc<RwLock<dyn Storage>>>,
    Path(file): Path<String>,
    Query(params): Query<PostUploadParams>,
    body: Bytes,
) -> Result<axum::Json<PostUploadResponse>, ApiError> {
    let storage_params = params.into_storage_parameters();
    let file_path = FilePath::new(&file);

    let storage = storage.read().await;
    storage
        .save(file_path, &body, &storage_params)
        .await
        .map_err(ApiError::StorageError)?;

    Ok(axum::Json(PostUploadResponse { file }))
}

/// Download a file from storage.
#[utoipa::path(
    get,
    path = "/download/{file}",
    params(
        ("file" = String, Path, description = "The file path/name to retrieve"),
    ),
    responses(
        (status = OK, description = "File successfully downloaded.", content_type = "application/octet-stream"),
        (status = INTERNAL_SERVER_ERROR, description = "Storage error."),
    ),
)]
pub(crate) async fn download(
    _user: OptionalUser,
    State(storage): State<Arc<RwLock<dyn Storage>>>,
    Path(file): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let file_path = FilePath::new(&file);

    let storage = storage.read().await;
    let content = storage
        .load(file_path)
        .await
        .map_err(ApiError::StorageError)?;

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        "application/octet-stream".parse().unwrap(),
    );

    Ok((headers, content))
}
