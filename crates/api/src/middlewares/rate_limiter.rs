use std::convert::Infallible;

use axum::http::Request;
use axum::response::{IntoResponse, Response};
use config::Config;
use tower::{Layer, Service};
use tower_governor::{GovernorError, GovernorLayer, governor::GovernorConfigBuilder};

use crate::error::ApiError;

/// Builds the rate-limiting middleware layer.
pub fn rate_limiter_layer<S, ReqBody>(
    config: &Config,
) -> impl Layer<
    S,
    Service: Service<
        Request<ReqBody>,
        Response = Response,
        Error = Infallible,
        Future: Send + 'static,
    > + Clone
                 + Send
                 + Sync
                 + 'static,
> + Clone
+ Send
+ Sync
+ 'static
where
    S: Service<Request<ReqBody>, Response = Response, Error = Infallible>
        + Clone
        + Send
        + Sync
        + 'static,
    S::Future: Send + 'static,
{
    let governor_config = GovernorConfigBuilder::default()
        .per_second(config.api.rate_limiter_refresh_per_second)
        .burst_size(config.api.rate_limiter_burst_size)
        // Advertise the rate-limit budget on every response via x-ratelimit-* headers
        .use_headers()
        .finish()
        .expect("rate limiter configuration must be valid");

    GovernorLayer::new(governor_config).error_handler(rate_limit_error_response)
}

/// Converts a rate-limiter rejection into the standard API response while keeping
/// the `retry-after` / `x-ratelimit-*` headers the limiter computed for the client.
pub(crate) fn rate_limit_error_response(error: GovernorError) -> Response {
    let headers = match &error {
        GovernorError::TooManyRequests { headers, .. }
        | GovernorError::Other { headers, .. } => headers.clone(),
        GovernorError::UnableToExtractKey => None,
    };
    let mut response = ApiError::from(error).into_response();
    if let Some(headers) = headers {
        response.headers_mut().extend(headers);
    }
    response
}
