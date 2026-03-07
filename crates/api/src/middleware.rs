use axum::http::StatusCode;
use axum::http::header::AUTHORIZATION;
use config::Config;
use std::iter::once;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::{
    CompressionLevel,
    compression::CompressionLayer,
    cors::CorsLayer,
    decompression::RequestDecompressionLayer,
    normalize_path::NormalizePathLayer,
    sensitive_headers::{
        SetSensitiveRequestHeadersLayer, SetSensitiveResponseHeadersLayer,
    },
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

pub(crate) fn middleware_layer<L>(
    config: &Config,
) -> ServiceBuilder<impl Clone + Send + Sync + 'static> {
    ServiceBuilder::new()
        // Avoid logging these headers content
        .layer(SetSensitiveRequestHeadersLayer::new(once(AUTHORIZATION)))
        .layer(SetSensitiveResponseHeadersLayer::new(once(AUTHORIZATION)))
        .layer(TraceLayer::new_for_http())
        // Authorize OPTIONS requests for CORS and automatically set up headers
        //TODO: Set this up based on what is actually available
        .layer(CorsLayer::permissive())
        .layer(NormalizePathLayer::trim_trailing_slash())
        .layer(CompressionLayer::new().quality(CompressionLevel::Best))
        .layer(RequestDecompressionLayer::new())
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(config.api.timeout_sec.into()),
        ))
}
