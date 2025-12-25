use crate::{api::misc::*, api::state::AppState};
use axum::{Router, routing::get};

use axum_prometheus::metrics_exporter_prometheus::PrometheusHandle;
use std::time::Duration;
use std::{future::ready, iter::once};
use tower::ServiceBuilder;
use tower_http::{
    CompressionLevel,
    compression::CompressionLayer,
    cors::CorsLayer,
    decompression::RequestDecompressionLayer,
    normalize_path::NormalizePathLayer,
    sensitive_headers::{SetSensitiveRequestHeadersLayer, SetSensitiveResponseHeadersLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use utoipa::openapi::{InfoBuilder, OpenApi};
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

//TODO: Put this in the config
const TIMEOUT_SEC: u64 = 20;

// Bookmark this: https://docs.rs/axum/latest/axum/routing/struct.Router.html

/// Public routes that qre exposed to the world
pub(crate) fn public_routes(app_state: &AppState) -> (Router, OpenApi) {
    let (api_routes, open_api) = OpenApiRouter::new()
        .routes(routes!(health_check))
        .split_for_parts();

    let middleware_service = ServiceBuilder::new()
        // Avoid logging these headers content
        // .layer(SetSensitiveRequestHeadersLayer::new(once(AUTHORIZATION)))
        // .layer(SetSensitiveResponseHeadersLayer::new(once(AUTHORIZATION)))
        .layer(TraceLayer::new_for_http())
        // Authorize OPTIONS requests for CORS and automatically set up headers
        //TODO: Set this up based on what is actually available
        .layer(CorsLayer::permissive())
        .layer(NormalizePathLayer::trim_trailing_slash())
        .layer(CompressionLayer::new().quality(CompressionLevel::Best))
        .layer(RequestDecompressionLayer::new())
        .layer(TimeoutLayer::new(Duration::from_secs(TIMEOUT_SEC)));

    let router = api_routes
        .layer(middleware_service)
        .with_state(app_state.clone());

    (router, open_api)
}

/// Private routes that are only exposed to the internal network
pub(crate) fn private_routes(_app_state: &AppState, openapi: OpenApi) -> Router {
    let swagger_config = _app_state.config.swagger.as_ref().unwrap().clone();
    let swagger_ui_path = swagger_config.swagger_ui_path;
    let openapi_path = swagger_config.openapi_path;
    let mut openapi = openapi;
    openapi.info = InfoBuilder::new()
        .title(env!("CARGO_PKG_NAME"))
        .description(option_env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .build();
    Router::new()
        .merge(SwaggerUi::new(swagger_ui_path).url(openapi_path, openapi))
        .layer(TraceLayer::new_for_http())
    // .layer(SetSensitiveRequestHeadersLayer::new(once(AUTHORIZATION)))
    // .layer(SetSensitiveResponseHeadersLayer::new(once(AUTHORIZATION)))
}

/// Metrics routes that are exposed to Prometheus
pub(crate) fn try_metrics_routes(metric_handle: PrometheusHandle) -> Result<Router, anyhow::Error> {
    Ok(Router::new().route("/metrics", get(move || ready(metric_handle.render()))))
}
