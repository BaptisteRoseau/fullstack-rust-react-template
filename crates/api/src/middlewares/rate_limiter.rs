use config::Config;
use tower_governor::{GovernorError, GovernorLayer, governor::GovernorConfigBuilder};

use crate::error::ApiError;

pub fn rate_limiter_layer(config: &Config) -> impl <what do I put here ?> {
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
pub(crate) fn rate_limit_error_response(
    error: GovernorError,
) -> axum::response::Response {
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
