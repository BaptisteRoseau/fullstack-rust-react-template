use tracing::Level;
use tracing_subscriber::EnvFilter;

/// Initializes the global JSON tracing subscriber.
///
/// Emits newline-delimited JSON so logs can be ingested by aggregators. Each
/// event carries the fields of the currently active span, so a per-request
/// span holding a `request_id` propagates that id onto every line logged while
/// handling the request, including events emitted by lower layers.
///
/// The level is taken from the `RUST_LOG` environment variable when set,
/// otherwise it defaults to `DEBUG` when `debug` is true and `INFO` otherwise.
pub fn init_logger(debug: bool) {
    let default_level = if debug { Level::DEBUG } else { Level::INFO };
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(default_level.to_string()));

    tracing_subscriber::fmt()
        .json()
        .with_current_span(true)
        .with_span_list(false)
        .with_env_filter(filter)
        .init();
}
