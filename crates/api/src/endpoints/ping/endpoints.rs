crate::endpoints::macros::declare_tag!("Health Check", "Service health checks.");

/// Health Check of the API
#[utoipa::path(
    get,
    path = "/ping",
    tag = TAG,
    responses(
        (status = OK, description = "The API is up and running."),
    ),
)]
pub(crate) async fn ping() -> &'static str {
    "pong"
}
