use crate::error::ApiError;

pub(crate) mod storage;

/// Upload a file.
#[utoipa::path(
    post,
    path = "/upload/{:file}",
    responses(
        (status = OK, description = "File have successfully been uploaded."),
    ),
)]
pub(crate) async fn upload() -> Result<&'static str, ApiError> {
    Ok("")
}

/// Download a file.
#[utoipa::path(
    get,
    path = "/download/{:file}",
    responses(
        (status = OK, description = "File successfully downloaded."),
    ),
)]
pub(crate) async fn download() -> Result<String, ApiError> {
    Ok("".into())
}
