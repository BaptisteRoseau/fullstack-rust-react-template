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
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{HeaderMap, StatusCode};
    use axum::routing::get;
    use axum::Router;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use std::time::Duration;
    use tower::ServiceBuilder;
    use tower::ServiceExt;
    use tower_http::request_id::{PropagateRequestIdLayer, SetRequestIdLayer};
    use tower_http::trace::TraceLayer;
    use tracing::field::{Field, Visit};
    use tracing_subscriber::layer::{Context, Layer};
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::registry::LookupSpan;

    const NONCE_HEADER: &str = "x-test-nonce";
    const NONCE_FIELD: &str = "nonce";

    /// Stored in a span's extensions: the `request_id` it was created with.
    struct SpanRequestId(String);

    /// Extracts a single named string field out of a span or event.
    struct FieldGrabber {
        target: &'static str,
        value: Option<String>,
    }

    impl Visit for FieldGrabber {
        fn record_str(&mut self, field: &Field, value: &str) {
            if field.name() == self.target {
                self.value = Some(value.to_owned());
            }
        }

        fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
            // `%value` (Display) fields arrive here as a `format_args`, whose
            // Debug rendering is the plain Display string with no quotes.
            if field.name() == self.target && self.value.is_none() {
                self.value = Some(format!("{value:?}"));
            }
        }
    }

    fn grab(target: &'static str, record: impl FnOnce(&mut FieldGrabber)) -> Option<String> {
        let mut grabber = FieldGrabber {
            target,
            value: None,
        };
        record(&mut grabber);
        grabber.value
    }

    /// Records, per request nonce, the `request_id` carried by the span that was
    /// active when the handler emitted its event.
    struct CaptureLayer {
        seen: Arc<Mutex<HashMap<String, String>>>,
    }

    impl<S> Layer<S> for CaptureLayer
    where
        S: tracing::Subscriber + for<'a> LookupSpan<'a>,
    {
        fn on_new_span(
            &self,
            attrs: &tracing::span::Attributes<'_>,
            id: &tracing::Id,
            ctx: Context<'_, S>,
        ) {
            if let Some(request_id) = grab("request_id", |g| attrs.record(g))
                && let Some(span) = ctx.span(id)
            {
                span.extensions_mut().insert(SpanRequestId(request_id));
            }
        }

        fn on_event(&self, event: &tracing::Event<'_>, ctx: Context<'_, S>) {
            let Some(nonce) = grab(NONCE_FIELD, |g| event.record(g)) else {
                return;
            };
            let Some(span) = ctx.event_span(event) else {
                return;
            };
            for span in span.scope() {
                if let Some(request_id) = span.extensions().get::<SpanRequestId>() {
                    self.seen.lock().unwrap().insert(nonce, request_id.0.clone());
                    return;
                }
            }
        }
    }

    /// Emits an event tagged with the request's nonce, after yielding so that
    /// concurrent requests interleave their execution on the runtime.
    async fn handler(headers: HeaderMap) -> StatusCode {
        let nonce = headers
            .get(NONCE_HEADER)
            .and_then(|value| value.to_str().ok())
            .unwrap_or_default()
            .to_owned();

        tokio::time::sleep(Duration::from_millis(10)).await;
        tokio::task::yield_now().await;

        tracing::info!(nonce = %nonce, "handler reached");
        StatusCode::OK
    }

    fn test_app() -> Router {
        Router::new().route("/", get(handler)).layer(
            ServiceBuilder::new()
                .layer(SetRequestIdLayer::x_request_id(MakeRequestUuidV7))
                .layer(TraceLayer::new_for_http().make_span_with(make_request_span))
                .layer(PropagateRequestIdLayer::x_request_id()),
        )
    }

    #[tokio::test]
    async fn concurrent_requests_keep_their_request_id() {
        let seen = Arc::new(Mutex::new(HashMap::<String, String>::new()));
        let subscriber = tracing_subscriber::registry().with(CaptureLayer {
            seen: Arc::clone(&seen),
        });
        let _guard = tracing::subscriber::set_default(subscriber);

        let app = test_app();
        let ids: Vec<String> = (0..8).map(|i| format!("req-{i}")).collect();

        let requests = ids.iter().cloned().map(|id| {
            let app = app.clone();
            async move {
                let request = axum::http::Request::builder()
                    .uri("/")
                    .header(REQUEST_ID_HEADER, &id)
                    .header(NONCE_HEADER, &id)
                    .body(Body::empty())
                    .unwrap();

                let response = app.oneshot(request).await.unwrap();

                let echoed = response
                    .headers()
                    .get(REQUEST_ID_HEADER)
                    .and_then(|value| value.to_str().ok())
                    .map(str::to_owned);
                assert_eq!(
                    echoed.as_deref(),
                    Some(id.as_str()),
                    "response should echo the request id it was sent, got {echoed:?} for {id}"
                );
            }
        });

        futures_join_all(requests).await;

        let seen = seen.lock().unwrap();
        for id in &ids {
            assert_eq!(
                seen.get(id),
                Some(id),
                "the handler event for nonce {id} must run under the span carrying request_id={id}, \
                 but the captured mapping was {seen:?}"
            );
        }
    }

    /// Drives every future concurrently to completion without pulling in an
    /// extra futures-util dependency; they interleave at their await points.
    async fn futures_join_all<F>(futures: impl IntoIterator<Item = F>)
    where
        F: std::future::Future<Output = ()> + 'static,
    {
        let mut set = tokio::task::JoinSet::new();
        let local = tokio::task::LocalSet::new();
        local
            .run_until(async move {
                for future in futures {
                    set.spawn_local(future);
                }
                while let Some(result) = set.join_next().await {
                    result.expect("a concurrent request task panicked");
                }
            })
            .await;
    }
}
