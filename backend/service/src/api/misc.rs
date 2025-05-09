/// Health Check of the API
#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = OK, description = "The API is up and running."),
    ),
)]
pub(crate) async fn health_check() -> &'static str {
    ""
}
