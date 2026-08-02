use super::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::Level;
use tracing::field::{Field, Visit};
use tracing_subscriber::layer::{Context, Layer};
use tracing_subscriber::prelude::*;

/// A tracing event as recorded by [`CaptureLayer`].
#[derive(Debug, Clone)]
struct CapturedEvent {
    level: Level,
    fields: HashMap<String, String>,
}

impl CapturedEvent {
    fn field(&self, name: &str) -> Option<&str> {
        self.fields.get(name).map(String::as_str)
    }
}

/// Collects every field of an event as its rendered string.
struct FieldCollector(HashMap<String, String>);

impl Visit for FieldCollector {
    fn record_str(&mut self, field: &Field, value: &str) {
        self.0.insert(field.name().to_owned(), value.to_owned());
    }

    fn record_debug(&mut self, field: &Field, value: &dyn Debug) {
        // `%value` (Display) fields arrive here as a `format_args`, whose
        // Debug rendering is the plain Display string with no quotes.
        self.0
            .entry(field.name().to_owned())
            .or_insert_with(|| format!("{value:?}"));
    }
}

/// Records every event emitted while it is the active subscriber.
struct CaptureLayer {
    events: Arc<Mutex<Vec<CapturedEvent>>>,
}

impl<S: tracing::Subscriber> Layer<S> for CaptureLayer {
    fn on_event(&self, event: &tracing::Event<'_>, _ctx: Context<'_, S>) {
        let mut collector = FieldCollector(HashMap::new());
        event.record(&mut collector);
        self.events.lock().unwrap().push(CapturedEvent {
            level: *event.metadata().level(),
            fields: collector.0,
        });
    }
}

/// Turns an error into its response and returns everything it logged.
fn events_logged_by(error: ApiError) -> Vec<CapturedEvent> {
    let events = Arc::new(Mutex::new(Vec::new()));
    let subscriber = tracing_subscriber::registry().with(CaptureLayer {
        events: Arc::clone(&events),
    });
    let _guard = tracing::subscriber::set_default(subscriber);

    let _response = error.into_response();

    events.lock().unwrap().clone()
}

#[test]
fn server_errors_are_logged_once_at_error_level() {
    let events =
        events_logged_by(ApiError::Unexpected(anyhow::anyhow!("database exploded")));

    assert_eq!(
        events.len(),
        1,
        "a 5xx must emit exactly one tracing event, got {} events: {events:?}",
        events.len()
    );
    assert_eq!(
        events[0].level,
        Level::ERROR,
        "a 5xx must be logged at ERROR, got {:?} in {events:?}",
        events[0].level
    );
    assert_eq!(
        events[0].field("status"),
        Some("500 Internal Server Error"),
        "the status must be a structured field, got {:?} in {events:?}",
        events[0].field("status")
    );
    let trace = events[0].field("error").unwrap_or_default();
    assert!(
        trace.contains("database exploded"),
        "the error field must carry the error trace, got {trace:?} in {events:?}"
    );
    assert!(
        !trace.starts_with('"'),
        "the error field must not be Debug-escaped into a quoted string, got {trace:?}"
    );
}

#[test]
fn client_errors_are_logged_once_at_warn_level() {
    let events = events_logged_by(ApiError::Unauthorized);

    assert_eq!(
        events.len(),
        1,
        "a 4xx must emit exactly one tracing event, got {} events: {events:?}",
        events.len()
    );
    assert_eq!(
        events[0].level,
        Level::WARN,
        "a 4xx must be logged at WARN, got {:?} in {events:?}",
        events[0].level
    );
    assert_eq!(
        events[0].field("status"),
        Some("401 Unauthorized"),
        "the status must be a structured field, got {:?} in {events:?}",
        events[0].field("status")
    );
}

#[test]
fn rate_limited_requests_are_logged_once_at_warn_level() {
    let events = events_logged_by(ApiError::TooManyRequests(30));

    assert_eq!(
        events.len(),
        1,
        "a 429 is a client error and must emit exactly one event, got {events:?}"
    );
    assert_eq!(
        events[0].level,
        Level::WARN,
        "a 429 must be logged at WARN, got {:?} in {events:?}",
        events[0].level
    );
}
