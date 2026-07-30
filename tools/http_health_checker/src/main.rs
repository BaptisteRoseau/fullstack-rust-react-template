use std::{env, process::ExitCode};

#[inline]
fn run(endpoint: &str) -> Result<minreq::Response, minreq::Error> {
    minreq::get(endpoint).with_timeout(3).send()
}

/// A target is healthy when it answers at all, with a non-error status.
fn is_healthy(endpoint: &str) -> bool {
    match run(endpoint) {
        Ok(response) => response.status_code <= 299,
        Err(_) => false,
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let endpoint = args.last().unwrap();
    if is_healthy(endpoint) {
        return ExitCode::from(0);
    }
    ExitCode::from(1)
}

#[cfg(test)]
mod tests {
    use std::net::TcpListener;
    use std::thread;

    use axum::Router;
    use axum::http::StatusCode;
    use axum::routing::get;

    use super::*;

    #[test]
    fn healthy_when_the_target_answers() {
        let url = format!("{}/ping", serve());
        assert!(
            is_healthy(&url),
            "the /ping route should be healthy, url={url}"
        );
    }

    #[test]
    fn unhealthy_when_the_target_answers_with_an_error() {
        let url = format!("{}/down", serve());
        assert!(
            !is_healthy(&url),
            "a 503 should be unhealthy, url={url} status={:?}",
            run(&url).map(|response| response.status_code)
        );
    }

    #[test]
    fn unhealthy_when_the_target_does_not_resolve() {
        let url = "https://weqweqwe.local/qwewqe";
        assert!(
            !is_healthy(url),
            "an unresolvable host should be unhealthy, url={url}"
        );
    }

    #[test]
    fn unhealthy_when_nothing_listens() {
        // Bound, then dropped: the port is free and nothing will answer on it.
        let port = TcpListener::bind("127.0.0.1:0")
            .expect("failed to reserve a port")
            .local_addr()
            .expect("failed to read the reserved address")
            .port();

        let url = format!("http://127.0.0.1:{port}/ping");
        assert!(
            !is_healthy(&url),
            "a closed port should be unhealthy, url={url}"
        );
    }

    /// Serves `/ping` and `/down` on an ephemeral port, and returns its base URL.
    ///
    /// The listener is bound before the thread starts, so the port is already
    /// accepting connections by the time the caller sends its request.
    fn serve() -> String {
        let listener =
            TcpListener::bind("127.0.0.1:0").expect("failed to bind the test server");
        let address = listener
            .local_addr()
            .expect("failed to read the test server address");
        listener
            .set_nonblocking(true)
            .expect("failed to make the test server non-blocking");

        thread::spawn(move || {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("failed to build the test server runtime");

            runtime.block_on(async move {
                let listener = tokio::net::TcpListener::from_std(listener)
                    .expect("failed to adopt the test server listener");
                let app = Router::new()
                    .route("/ping", get(|| async { "pong" }))
                    .route("/down", get(|| async { StatusCode::SERVICE_UNAVAILABLE }));

                axum::serve(listener, app)
                    .await
                    .expect("the test server stopped");
            });
        });

        format!("http://{address}")
    }
}
