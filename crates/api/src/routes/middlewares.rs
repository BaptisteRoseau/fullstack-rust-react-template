use super::observability::{MakeRequestUuidV7, make_request_span};
use crate::middlewares::rate_limiter::rate_limiter_layer;
use axum::Router;
use axum::body::Body;
use axum::http::Request;
use axum::http::StatusCode;
use axum::http::header::AUTHORIZATION;
use config::Config;
use std::iter::once;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::{
    CompressionLevel,
    compression::CompressionLayer,
    cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer},
    decompression::RequestDecompressionLayer,
    normalize_path::NormalizePathLayer,
    request_id::{PropagateRequestIdLayer, SetRequestIdLayer},
    sensitive_headers::{
        SetSensitiveRequestHeadersLayer, SetSensitiveResponseHeadersLayer,
    },
    timeout::TimeoutLayer,
    trace::{DefaultOnRequest, DefaultOnResponse, HttpMakeClassifier, TraceLayer},
};
use tracing::{Level, Span};

/// Wraps `routes` in the production middleware stack.
///
/// Extracted from [`super::router::public_routes`] so the exact layer ordering can be
/// exercised by tests without standing up an [`crate::AppState`]: a test that hand-rolls
/// its own stack proves nothing about the one that ships.
pub(crate) fn with_middlewares<S>(routes: Router<S>, config: &Config) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    // `ServiceBuilder` wraps outside-in: the first layer declared here is the
    // outermost one, so a request traverses them top to bottom and the response
    // travels back up bottom to top. Order is load-bearing below.
    let middleware = ServiceBuilder::new()
        // Outermost, so no later layer can log the request's Authorization header
        .layer(SetSensitiveRequestHeadersLayer::new(once(AUTHORIZATION)))
        // Reuse an inbound x-request-id or mint a fresh UUID v7. Must wrap the trace
        // layer so the span can read the id from the request headers.
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuidV7))
        // Echo the request id back to the client in the response headers
        .layer(PropagateRequestIdLayer::x_request_id())
        // Scope every event of the request under a span carrying the request_id and log request/response.
        .layer(logging_layer())
        // Inside the trace layer, so responses are scrubbed before it sees them
        .layer(SetSensitiveResponseHeadersLayer::new(once(AUTHORIZATION)))
        // Reflect the frontend origin and allow credentials so the browser sends and
        // accepts the httpOnly auth cookies (a wildcard origin is rejected with creds).
        // Wraps the rate limiter so even a 429 carries the CORS headers the browser
        // needs to read it.
        .layer(cors_layer(config))
        .layer(NormalizePathLayer::trim_trailing_slash())
        // Bounds everything below it, decompression included
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(config.api.timeout_sec.into()),
        ))
        .layer(RequestDecompressionLayer::new())
        .layer(CompressionLayer::new().quality(CompressionLevel::Best))
        // Innermost: the governor layer only accepts a service answering a plain
        // `axum::response::Response`, which the compression layers above rewrap.
        // It still rejects abusive callers before the router picks a handler, and
        // its 429 travels back out through the CORS and tracing layers.
        .layer(rate_limiter_layer(config));

    routes.layer(middleware)
}

/// Type of the tracing layer built by [`logging_layer`].
type LoggingLayer =
    TraceLayer<HttpMakeClassifier, fn(&Request<Body>) -> Span, DefaultOnRequest, DefaultOnResponse>;

/// Tracing layer that scopes every request/response log under the span built by
/// [`make_request_span`] (carrying the `request_id`, method and path).
fn logging_layer() -> LoggingLayer {
    TraceLayer::new_for_http()
        .make_span_with(make_request_span as fn(&Request<Body>) -> Span)
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO))
}

/// CORS layer reflecting the configured frontend origin and allowing credentials,
/// which is required for the browser to send/accept the httpOnly auth cookies.
fn cors_layer(config: &Config) -> CorsLayer {
    let origin = config
        .oidc
        .frontend_url
        .trim_end_matches('/')
        .parse::<axum::http::HeaderValue>();

    let mut cors = CorsLayer::new()
        .allow_methods(AllowMethods::mirror_request())
        .allow_headers(AllowHeaders::mirror_request())
        .allow_credentials(true);

    cors = match origin {
        Ok(origin) => cors.allow_origin(origin),
        Err(_) => cors.allow_origin(AllowOrigin::mirror_request()),
    };

    cors
}
