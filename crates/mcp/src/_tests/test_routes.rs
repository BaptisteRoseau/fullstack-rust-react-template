use super::*;
use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use database::testing::MockDatabase;
use http_body_util::BodyExt;
use tower::ServiceExt;

/// The `initialize` call every MCP client opens with, as a raw JSON-RPC body.
const INITIALIZE_BODY: &str = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{
    "protocolVersion":"2025-06-18",
    "capabilities":{},
    "clientInfo":{"name":"test","version":"0"}
}}"#;

fn test_mcp_config(path: &str) -> McpConfig {
    McpConfig {
        path: path.to_string(),
        allowed_hosts: vec!["localhost".to_string()],
        json_response: true,
    }
}

fn test_router(path: &str) -> Router {
    let state = McpState::new(Arc::new(tokio::sync::RwLock::new(MockDatabase::default())));
    mcp_routes::<()>(&test_mcp_config(path), state)
}

fn initialize_request(path: &str) -> Request<Body> {
    Request::post(path)
        .header(header::HOST, "localhost")
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::ACCEPT, "application/json, text/event-stream")
        .body(Body::from(INITIALIZE_BODY))
        .expect("building the initialize request")
}

#[tokio::test]
async fn test_endpoint_answers_on_the_configured_path() {
    let response = test_router("/mcp")
        .oneshot(initialize_request("/mcp"))
        .await
        .expect("the router is infallible");

    let status = response.status();
    let body = response
        .into_body()
        .collect()
        .await
        .expect("reading the response body")
        .to_bytes();
    let body = String::from_utf8_lossy(&body);

    assert_eq!(status, StatusCode::OK, "initialize was refused: {body}");
    assert!(
        body.contains("serverInfo"),
        "the handshake should answer with the server info, got {body}"
    );
}

#[tokio::test]
async fn test_nothing_is_mounted_outside_the_configured_path() {
    let response = test_router("/mcp")
        .oneshot(initialize_request("/somewhere-else"))
        .await
        .expect("the router is infallible");

    assert_eq!(
        response.status(),
        StatusCode::NOT_FOUND,
        "the crate must expose exactly one path, but /somewhere-else answered {}",
        response.status()
    );
}

#[tokio::test]
async fn test_a_foreign_host_header_is_rejected() {
    let request = Request::post("/mcp")
        .header(header::HOST, "evil.example.com")
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::ACCEPT, "application/json, text/event-stream")
        .body(Body::from(INITIALIZE_BODY))
        .expect("building the initialize request");

    let response = test_router("/mcp")
        .oneshot(request)
        .await
        .expect("the router is infallible");

    assert!(
        response.status().is_client_error(),
        "a Host outside the allow list must be refused, got {}",
        response.status()
    );
}
