//! Request-scoped tracing helpers.
//!
//! These wire a `request_id` onto a per-request tracing span so that every
//! event logged while handling the request — including ones emitted by lower
//! layers such as the database — inherits the id without any context being
//! threaded through function signatures.

use axum::http::{HeaderValue, Request};
use tower_http::request_id::{MakeRequestId, RequestId};
use tracing::Span;
use uuid::Uuid;

/// Header carrying the request id, both inbound and on the echoed response.
pub(crate) const REQUEST_ID_HEADER: &str = "x-request-id";

/// Generates request ids as UUID v7, matching the project-wide id convention.
///
/// Only used when an inbound request does not already provide an
/// `x-request-id` header.
#[derive(Clone, Default)]
pub(crate) struct MakeRequestUuidV7;

impl MakeRequestId for MakeRequestUuidV7 {
    fn make_request_id<B>(&mut self, _request: &Request<B>) -> Option<RequestId> {
        let id = Uuid::now_v7().to_string();
        HeaderValue::from_str(&id).ok().map(RequestId::new)
    }
}

/// Builds the per-request span carrying the `request_id`, method and path.
///
/// Runs after the request-id layer, so the `x-request-id` header is already
/// present (either propagated from the caller or freshly generated).
pub(crate) fn make_request_span<B>(request: &Request<B>) -> Span {
    let request_id = request
        .headers()
        .get(REQUEST_ID_HEADER)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("unknown");

    tracing::info_span!(
        "http_request",
        request_id = %request_id,
        method = %request.method(),
        uri = %request.uri().path(),
        api_version = super::openapi::api_version(),
        // Populated later by the `UserToken` extractor, once the request has been
        // authenticated; stays absent for anonymous requests.
        user_id = tracing::field::Empty,
    )
}

test_utils::tests_file!("_tests/test_observability.rs");
