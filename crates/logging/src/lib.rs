use tracing::Level;
use tracing_subscriber::EnvFilter;

/// Initializes the global tracing subscriber.
///
/// When `json` is true, emits newline-delimited JSON so logs can be ingested by
/// aggregators; otherwise emits a human-readable compact format. Each event
/// carries the fields of the currently active span, so a per-request span
/// holding a `request_id` propagates that id onto every line logged while
/// handling the request, including events emitted by lower layers.
///
/// The level is taken from the `RUST_LOG` environment variable when set,
/// otherwise it defaults to `DEBUG` when `debug` is true and `INFO` otherwise.
pub fn init_logger(debug: bool, json: bool) {
    let default_level = if debug { Level::DEBUG } else { Level::INFO };
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(default_level.to_string()));

    let builder = tracing_subscriber::fmt()
        .log_internal_errors(true)
        .with_line_number(true)
        .with_env_filter(filter);

    if json {
        builder
            .json()
            .with_current_span(true)
            .with_span_list(false)
            .init();
    } else {
        builder.compact().init();
    }
}
