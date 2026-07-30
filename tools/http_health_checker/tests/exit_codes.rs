//! Runs the compiled binary against a local server: the exit code is the whole
//! contract, and `main` is the only place it is decided.

use std::net::TcpListener;
use std::process::Command;
use std::thread;

use axum::Router;
use axum::http::StatusCode;
use axum::routing::get;

#[test]
fn answering_target_exits_0() {
    let url = format!("{}/ping", serve());

    let code = exit_code(&url);

    assert_eq!(code, 0, "a 200 should be healthy, url={url} exit={code}");
}

#[test]
fn error_status_exits_1() {
    let url = format!("{}/down", serve());

    let code = exit_code(&url);

    assert_eq!(code, 1, "a 503 should be unhealthy, url={url} exit={code}");
}

#[test]
fn nonsense_exits_1() {
    let url = "https://weqweqwe.local/qwewqe";
    let code = exit_code(url);
    assert_eq!(
        code, 1,
        "a host that does not resolve should be unhealthy, url={url} exit={code}"
    );
}

/// Runs the health checker the way a container's `HEALTHCHECK` does.
fn exit_code(endpoint: &str) -> i32 {
    Command::new(env!("CARGO_BIN_EXE_http_health_checker"))
        .arg(endpoint)
        .status()
        .expect("failed to run the health checker")
        .code()
        .expect("the health checker was killed by a signal")
}

/// Serves `/ping` and `/down` on an ephemeral port, and returns its base URL.
///
/// The listener is bound before the thread starts, so the port already accepts
/// connections by the time the caller runs the binary against it.
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
