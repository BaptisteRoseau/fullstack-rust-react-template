use std::sync::Arc;

use axum::{
    extract::{Multipart, Path, Query, State},
    http::{HeaderMap, StatusCode, header},
    response::{IntoResponse, Json},
};
use config::Config;
use storage::Storage;
use tokio::sync::RwLock;
use uuid::Uuid;

use super::models::{
    GetDirectoryResponse, GetEntriesParams, GetEntriesResponse, GetFileResponse,
    GetPermissionResponse, PatchEntryRequest, PostDirectoryRequest, PostUploadFileParams,
    PutPermissionRequest,
};
use crate::{
    app_state::AppState,
    error::{ApiError, ApiErrorResponse},
    models::UserToken,
};

crate::endpoints::macros::declare_tag!(
    "Files",
    "Browse, upload, share and download files stored compressed and encrypted."
);

/// Name of the multipart field carrying the uploaded bytes.
const UPLOAD_FIELD: &str = "file";

/// Fallback when the client sends no content type for an uploaded file.
const DEFAULT_MIME_TYPE: &str = "application/octet-stream";

/// List the direct children of a directory. Omit `parentId` to list the
/// caller's own root. Requires at least viewer access on the directory.
#[axum_macros::debug_handler(state = AppState)]
#[utoipa::path(
    get,
    path = "/files",
    tag = TAG,
    params(GetEntriesParams),
    responses(
        (status = OK, body = GetEntriesResponse, description = "The directory and its direct children."),
        (status = NOT_FOUND, body = ApiErrorResponse, description = "No such directory, or not visible to the caller."),
        (status = UNAUTHORIZED, body = ApiErrorResponse, description = "Not authenticated."),
    ),
)]
pub(crate) async fn list_entries(
    user: UserToken,
    State(db): State<Arc<RwLock<dyn database::Database>>>,
    Query(params): Query<GetEntriesParams>,
) -> Result<Json<GetEntriesResponse>, ApiError> {
    let listing = {
        let db = db.read().await;
        app_core::directory::list_entries(&*db, user.id, params.parent_id).await?
    };

    Ok(Json(GetEntriesResponse {
        directory: listing.directory.map(GetDirectoryResponse::from),
        directories: listing
            .directories
            .into_iter()
            .map(GetDirectoryResponse::from)
            .collect(),
        files: listing
            .files
            .into_iter()
            .map(GetFileResponse::from)
            .collect(),
    }))
}

/// Create a directory. Requires at least editor access on the parent; omitting
/// `parentId` creates it at the caller's own root.
#[axum_macros::debug_handler(state = AppState)]
#[utoipa::path(
    post,
    path = "/files/directories",
    tag = TAG,
    request_body = PostDirectoryRequest,
    responses(
        (status = CREATED, body = GetDirectoryResponse, description = "Directory created."),
        (status = BAD_REQUEST, body = ApiErrorResponse, description = "The name is empty, too long or holds a path separator."),
        (status = NOT_FOUND, body = ApiErrorResponse, description = "No such parent, or not writable by the caller."),
        (status = UNAUTHORIZED, body = ApiErrorResponse, description = "Not authenticated."),
    ),
)]
pub(crate) async fn create_directory(
    user: UserToken,
    State(db): State<Arc<RwLock<dyn database::Database>>>,
    Json(body): Json<PostDirectoryRequest>,
) -> Result<(StatusCode, Json<GetDirectoryResponse>), ApiError> {
    let directory = {
        let mut db = db.write().await;
        app_core::directory::create_directory(
            &mut *db,
            user.id,
            body.name,
            body.parent_id,
        )
        .await?
    };

    Ok((
        StatusCode::CREATED,
        Json(GetDirectoryResponse::from(directory)),
    ))
}

/// Rename and/or move a directory. Requires at least editor access on it, and
/// on the destination when it moves.
#[axum_macros::debug_handler(state = AppState)]
#[utoipa::path(
    patch,
    path = "/files/directories/{id}",
    tag = TAG,
    params(("id" = Uuid, Path, description = "Directory ID")),
    request_body = PatchEntryRequest,
    responses(
        (status = OK, body = GetDirectoryResponse, description = "The updated directory."),
        (status = BAD_REQUEST, body = ApiErrorResponse, description = "Invalid name, or the directory would be moved inside itself."),
        (status = NOT_FOUND, body = ApiErrorResponse, description = "No such directory, or not writable by the caller."),
        (status = UNAUTHORIZED, body = ApiErrorResponse, description = "Not authenticated."),
    ),
)]
pub(crate) async fn update_directory(
    user: UserToken,
    State(db): State<Arc<RwLock<dyn database::Database>>>,
    Path(id): Path<Uuid>,
    Json(body): Json<PatchEntryRequest>,
) -> Result<Json<GetDirectoryResponse>, ApiError> {
    let directory = {
        let mut db = db.write().await;
        app_core::directory::update_directory(
            &mut *db,
            user.id,
            id,
            body.name,
            body.parent_id,
        )
        .await?
    };

    Ok(Json(GetDirectoryResponse::from(directory)))
}

/// Delete a directory and everything below it. Requires manager access.
#[axum_macros::debug_handler(state = AppState)]
#[utoipa::path(
    delete,
    path = "/files/directories/{id}",
    tag = TAG,
    params(("id" = Uuid, Path, description = "Directory ID")),
    responses(
        (status = NO_CONTENT, description = "Directory deleted."),
        (status = NOT_FOUND, body = ApiErrorResponse, description = "No such directory, or not managed by the caller."),
        (status = UNAUTHORIZED, body = ApiErrorResponse, description = "Not authenticated."),
    ),
)]
pub(crate) async fn delete_directory(
    user: UserToken,
    State(db): State<Arc<RwLock<dyn database::Database>>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    {
        let mut db = db.write().await;
        app_core::directory::delete_directory(&mut *db, user.id, id).await?;
    }

    Ok(StatusCode::NO_CONTENT)
}

/// Upload a file as `multipart/form-data` under the field `file`. The content is
/// compressed then encrypted before it reaches the object store. Requires at
/// least editor access on the target directory.
#[axum_macros::debug_handler(state = AppState)]
#[utoipa::path(
    post,
    path = "/files/upload",
    tag = TAG,
    params(PostUploadFileParams),
    request_body(content = String, content_type = "multipart/form-data", description = "A `file` field holding the content to store."),
    responses(
        (status = CREATED, body = GetFileResponse, description = "File stored."),
        (status = BAD_REQUEST, body = ApiErrorResponse, description = "No `file` field, or an unusable name."),
        (status = NOT_FOUND, body = ApiErrorResponse, description = "No such directory, or not writable by the caller."),
        (status = UNAUTHORIZED, body = ApiErrorResponse, description = "Not authenticated."),
    ),
)]
pub(crate) async fn upload_file(
    user: UserToken,
    State(db): State<Arc<RwLock<dyn database::Database>>>,
    State(blobs): State<Arc<RwLock<dyn Storage>>>,
    State(config): State<Arc<Config>>,
    Query(params): Query<PostUploadFileParams>,
    multipart: Multipart,
) -> Result<(StatusCode, Json<GetFileResponse>), ApiError> {
    let upload = read_upload(multipart).await?;

    let file = {
        let blobs = blobs.read().await;
        let mut db = db.write().await;
        app_core::file::upload_file(
            &mut *db,
            &*blobs,
            &config.storage.encryption_key,
            user.id,
            params.parent_id,
            upload,
        )
        .await?
    };

    Ok((StatusCode::CREATED, Json(GetFileResponse::from(file))))
}

/// Get a file's metadata. Requires at least viewer access.
#[axum_macros::debug_handler(state = AppState)]
#[utoipa::path(
    get,
    path = "/files/{id}",
    tag = TAG,
    params(("id" = Uuid, Path, description = "File ID")),
    responses(
        (status = OK, body = GetFileResponse, description = "The file's metadata."),
        (status = NOT_FOUND, body = ApiErrorResponse, description = "No such file, or not visible to the caller."),
        (status = UNAUTHORIZED, body = ApiErrorResponse, description = "Not authenticated."),
    ),
)]
pub(crate) async fn get_file(
    user: UserToken,
    State(db): State<Arc<RwLock<dyn database::Database>>>,
    Path(id): Path<Uuid>,
) -> Result<Json<GetFileResponse>, ApiError> {
    let file = {
        let db = db.read().await;
        app_core::file::read_file(&*db, user.id, id).await?
    };

    Ok(Json(GetFileResponse::from(file)))
}

/// Download a file's original content, decrypted and decompressed. Requires at
/// least viewer access.
#[axum_macros::debug_handler(state = AppState)]
#[utoipa::path(
    get,
    path = "/files/{id}/download",
    tag = TAG,
    params(("id" = Uuid, Path, description = "File ID")),
    responses(
        (status = OK, description = "The file's content.", content_type = "application/octet-stream"),
        (status = NOT_FOUND, body = ApiErrorResponse, description = "No such file, or not visible to the caller."),
        (status = UNAUTHORIZED, body = ApiErrorResponse, description = "Not authenticated."),
    ),
)]
pub(crate) async fn download_file(
    user: UserToken,
    State(db): State<Arc<RwLock<dyn database::Database>>>,
    State(blobs): State<Arc<RwLock<dyn Storage>>>,
    State(config): State<Arc<Config>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let (file, content) = {
        let db = db.read().await;
        let blobs = blobs.read().await;
        app_core::file::download_file(
            &*db,
            &*blobs,
            &config.storage.encryption_key,
            user.id,
            id,
        )
        .await?
    };

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        file.mime_type
            .parse()
            .unwrap_or_else(|_| DEFAULT_MIME_TYPE.parse().expect("a valid header value")),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        content_disposition(&file.name)
            .parse()
            .map_err(|_| ApiError::NotFound(id.to_string()))?,
    );

    Ok((headers, content))
}

/// Download a file's WebP thumbnail. Only images have one. Requires at least
/// viewer access.
#[axum_macros::debug_handler(state = AppState)]
#[utoipa::path(
    get,
    path = "/files/{id}/thumbnail",
    tag = TAG,
    params(("id" = Uuid, Path, description = "File ID")),
    responses(
        (status = OK, description = "The thumbnail.", content_type = "image/webp"),
        (status = NOT_FOUND, body = ApiErrorResponse, description = "No such file, no thumbnail, or not visible to the caller."),
        (status = UNAUTHORIZED, body = ApiErrorResponse, description = "Not authenticated."),
    ),
)]
pub(crate) async fn download_thumbnail(
    user: UserToken,
    State(db): State<Arc<RwLock<dyn database::Database>>>,
    State(blobs): State<Arc<RwLock<dyn Storage>>>,
    State(config): State<Arc<Config>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let thumbnail = {
        let db = db.read().await;
        let blobs = blobs.read().await;
        app_core::file::download_thumbnail(
            &*db,
            &*blobs,
            &config.storage.encryption_key,
            user.id,
            id,
        )
        .await?
    };

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        "image/webp".parse().expect("a valid header value"),
    );

    Ok((headers, thumbnail))
}

/// Rename and/or move a file. Requires at least editor access on it, and on the
/// destination directory when it moves.
#[axum_macros::debug_handler(state = AppState)]
#[utoipa::path(
    patch,
    path = "/files/{id}",
    tag = TAG,
    params(("id" = Uuid, Path, description = "File ID")),
    request_body = PatchEntryRequest,
    responses(
        (status = OK, body = GetFileResponse, description = "The updated file."),
        (status = BAD_REQUEST, body = ApiErrorResponse, description = "The name is empty, too long or holds a path separator."),
        (status = NOT_FOUND, body = ApiErrorResponse, description = "No such file, or not writable by the caller."),
        (status = UNAUTHORIZED, body = ApiErrorResponse, description = "Not authenticated."),
    ),
)]
pub(crate) async fn update_file(
    user: UserToken,
    State(db): State<Arc<RwLock<dyn database::Database>>>,
    Path(id): Path<Uuid>,
    Json(body): Json<PatchEntryRequest>,
) -> Result<Json<GetFileResponse>, ApiError> {
    let file = {
        let mut db = db.write().await;
        app_core::file::update_file(&mut *db, user.id, id, body.name, body.parent_id)
            .await?
    };

    Ok(Json(GetFileResponse::from(file)))
}

/// Delete a file and the blobs behind it. Requires manager access.
#[axum_macros::debug_handler(state = AppState)]
#[utoipa::path(
    delete,
    path = "/files/{id}",
    tag = TAG,
    params(("id" = Uuid, Path, description = "File ID")),
    responses(
        (status = NO_CONTENT, description = "File deleted."),
        (status = NOT_FOUND, body = ApiErrorResponse, description = "No such file, or not managed by the caller."),
        (status = UNAUTHORIZED, body = ApiErrorResponse, description = "Not authenticated."),
    ),
)]
pub(crate) async fn delete_file(
    user: UserToken,
    State(db): State<Arc<RwLock<dyn database::Database>>>,
    State(blobs): State<Arc<RwLock<dyn Storage>>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    {
        let blobs = blobs.read().await;
        let mut db = db.write().await;
        app_core::file::delete_file(&mut *db, &*blobs, user.id, id).await?;
    }

    Ok(StatusCode::NO_CONTENT)
}

/// List the grants on a file. Requires manager access.
#[axum_macros::debug_handler(state = AppState)]
#[utoipa::path(
    get,
    path = "/files/{id}/permissions",
    tag = TAG,
    params(("id" = Uuid, Path, description = "File ID")),
    responses(
        (status = OK, body = Vec<GetPermissionResponse>, description = "The grants on the file."),
        (status = NOT_FOUND, body = ApiErrorResponse, description = "No such file, or not managed by the caller."),
        (status = UNAUTHORIZED, body = ApiErrorResponse, description = "Not authenticated."),
    ),
)]
pub(crate) async fn list_file_permissions(
    user: UserToken,
    State(db): State<Arc<RwLock<dyn database::Database>>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<GetPermissionResponse>>, ApiError> {
    let permissions = {
        let db = db.read().await;
        app_core::sharing::list_file_permissions(&*db, user.id, id).await?
    };

    Ok(Json(
        permissions
            .into_iter()
            .map(GetPermissionResponse::from)
            .collect(),
    ))
}

/// Grant a user a level on a file, or change the level it already holds.
/// Requires manager access, and no caller may grant above its own level.
#[axum_macros::debug_handler(state = AppState)]
#[utoipa::path(
    put,
    path = "/files/{id}/permissions/{userId}",
    tag = TAG,
    params(
        ("id" = Uuid, Path, description = "File ID"),
        ("userId" = Uuid, Path, description = "The user to grant the level to"),
    ),
    request_body = PutPermissionRequest,
    responses(
        (status = OK, body = GetPermissionResponse, description = "The grant."),
        (status = BAD_REQUEST, body = ApiErrorResponse, description = "Granting to oneself, or above the caller's own level."),
        (status = NOT_FOUND, body = ApiErrorResponse, description = "No such file or user, or not managed by the caller."),
        (status = UNAUTHORIZED, body = ApiErrorResponse, description = "Not authenticated."),
    ),
)]
pub(crate) async fn grant_file_permission(
    user: UserToken,
    State(db): State<Arc<RwLock<dyn database::Database>>>,
    Path((id, grantee)): Path<(Uuid, Uuid)>,
    Json(body): Json<PutPermissionRequest>,
) -> Result<Json<GetPermissionResponse>, ApiError> {
    let permission = {
        let mut db = db.write().await;
        app_core::sharing::grant_file_permission(
            &mut *db,
            user.id,
            id,
            grantee,
            body.level.into(),
        )
        .await?
    };

    Ok(Json(GetPermissionResponse::from(permission)))
}

/// Revoke a user's grant on a file. Requires manager access. Access inherited
/// from an ancestor directory is unaffected; revoke it on that directory.
#[axum_macros::debug_handler(state = AppState)]
#[utoipa::path(
    delete,
    path = "/files/{id}/permissions/{userId}",
    tag = TAG,
    params(
        ("id" = Uuid, Path, description = "File ID"),
        ("userId" = Uuid, Path, description = "The user whose grant is revoked"),
    ),
    responses(
        (status = NO_CONTENT, description = "Grant revoked."),
        (status = NOT_FOUND, body = ApiErrorResponse, description = "No such grant or file, or not managed by the caller."),
        (status = UNAUTHORIZED, body = ApiErrorResponse, description = "Not authenticated."),
    ),
)]
pub(crate) async fn revoke_file_permission(
    user: UserToken,
    State(db): State<Arc<RwLock<dyn database::Database>>>,
    Path((id, grantee)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, ApiError> {
    {
        let mut db = db.write().await;
        app_core::sharing::revoke_file_permission(&mut *db, user.id, id, grantee).await?;
    }

    Ok(StatusCode::NO_CONTENT)
}

/// List the grants on a directory. Requires manager access.
#[axum_macros::debug_handler(state = AppState)]
#[utoipa::path(
    get,
    path = "/files/directories/{id}/permissions",
    tag = TAG,
    params(("id" = Uuid, Path, description = "Directory ID")),
    responses(
        (status = OK, body = Vec<GetPermissionResponse>, description = "The grants on the directory."),
        (status = NOT_FOUND, body = ApiErrorResponse, description = "No such directory, or not managed by the caller."),
        (status = UNAUTHORIZED, body = ApiErrorResponse, description = "Not authenticated."),
    ),
)]
pub(crate) async fn list_directory_permissions(
    user: UserToken,
    State(db): State<Arc<RwLock<dyn database::Database>>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<GetPermissionResponse>>, ApiError> {
    let permissions = {
        let db = db.read().await;
        app_core::sharing::list_directory_permissions(&*db, user.id, id).await?
    };

    Ok(Json(
        permissions
            .into_iter()
            .map(GetPermissionResponse::from)
            .collect(),
    ))
}

/// Grant a user a level on a directory and, by inheritance, on everything below
/// it. Requires manager access, and no caller may grant above its own level.
#[axum_macros::debug_handler(state = AppState)]
#[utoipa::path(
    put,
    path = "/files/directories/{id}/permissions/{userId}",
    tag = TAG,
    params(
        ("id" = Uuid, Path, description = "Directory ID"),
        ("userId" = Uuid, Path, description = "The user to grant the level to"),
    ),
    request_body = PutPermissionRequest,
    responses(
        (status = OK, body = GetPermissionResponse, description = "The grant."),
        (status = BAD_REQUEST, body = ApiErrorResponse, description = "Granting to oneself, or above the caller's own level."),
        (status = NOT_FOUND, body = ApiErrorResponse, description = "No such directory or user, or not managed by the caller."),
        (status = UNAUTHORIZED, body = ApiErrorResponse, description = "Not authenticated."),
    ),
)]
pub(crate) async fn grant_directory_permission(
    user: UserToken,
    State(db): State<Arc<RwLock<dyn database::Database>>>,
    Path((id, grantee)): Path<(Uuid, Uuid)>,
    Json(body): Json<PutPermissionRequest>,
) -> Result<Json<GetPermissionResponse>, ApiError> {
    let permission = {
        let mut db = db.write().await;
        app_core::sharing::grant_directory_permission(
            &mut *db,
            user.id,
            id,
            grantee,
            body.level.into(),
        )
        .await?
    };

    Ok(Json(GetPermissionResponse::from(permission)))
}

/// Revoke a user's grant on a directory. Requires manager access.
#[axum_macros::debug_handler(state = AppState)]
#[utoipa::path(
    delete,
    path = "/files/directories/{id}/permissions/{userId}",
    tag = TAG,
    params(
        ("id" = Uuid, Path, description = "Directory ID"),
        ("userId" = Uuid, Path, description = "The user whose grant is revoked"),
    ),
    responses(
        (status = NO_CONTENT, description = "Grant revoked."),
        (status = NOT_FOUND, body = ApiErrorResponse, description = "No such grant or directory, or not managed by the caller."),
        (status = UNAUTHORIZED, body = ApiErrorResponse, description = "Not authenticated."),
    ),
)]
pub(crate) async fn revoke_directory_permission(
    user: UserToken,
    State(db): State<Arc<RwLock<dyn database::Database>>>,
    Path((id, grantee)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, ApiError> {
    {
        let mut db = db.write().await;
        app_core::sharing::revoke_directory_permission(&mut *db, user.id, id, grantee)
            .await?;
    }

    Ok(StatusCode::NO_CONTENT)
}

/// Pulls the `file` field out of a multipart body, along with the name and
/// content type the client attached to it.
async fn read_upload(
    mut multipart: Multipart,
) -> Result<app_core::file::Upload, ApiError> {
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        ApiError::CoreError(app_core::error::CoreError::InvalidRequest(format!(
            "malformed multipart body: {e}"
        )))
    })? {
        if field.name() != Some(UPLOAD_FIELD) {
            continue;
        }

        let name = field.file_name().unwrap_or_default().to_string();
        let mime_type = field
            .content_type()
            .unwrap_or(DEFAULT_MIME_TYPE)
            .to_string();
        let content = field.bytes().await.map_err(|e| {
            ApiError::CoreError(app_core::error::CoreError::InvalidRequest(format!(
                "could not read the uploaded content: {e}"
            )))
        })?;

        return Ok(app_core::file::Upload {
            name,
            mime_type,
            content: content.to_vec(),
        });
    }

    Err(ApiError::CoreError(
        app_core::error::CoreError::InvalidRequest(format!(
            "the multipart body carries no `{UPLOAD_FIELD}` field"
        )),
    ))
}

/// Quotes the stored name for `Content-Disposition`. A double quote or a
/// backslash inside it would end the quoted string early, so both are dropped
/// rather than escaped: this header has no reliable escaping across clients.
fn content_disposition(name: &str) -> String {
    let sanitized: String = name
        .chars()
        .filter(|c| *c != '"' && *c != '\\' && !c.is_control())
        .collect();
    format!("attachment; filename=\"{sanitized}\"")
}
