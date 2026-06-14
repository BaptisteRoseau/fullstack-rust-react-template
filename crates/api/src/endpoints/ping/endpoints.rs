/// Health Check of the API
#[utoipa::path(
    get,
    path = "/ping",
    responses(
        (status = OK, description = "The API is up and running."),
    ),
)]
pub(crate) async fn ping() -> &'static str {
    "pong"
}
