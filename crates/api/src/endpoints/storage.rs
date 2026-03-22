use crate::{error::ApiError, extractors::OptionalUser};

/// Upload a file.
#[utoipa::path(
    post,
    path = "/upload/{:file}",
    responses(
        (status = OK, description = "File have successfully been uploaded."),
    ),
)]
pub(crate) async fn upload(user: OptionalUser) -> Result<&'static str, ApiError> {
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
pub(crate) async fn download(user: OptionalUser) -> Result<String, ApiError> {
    Ok("".into())
}
