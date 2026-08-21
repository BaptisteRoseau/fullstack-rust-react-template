use super::*;

#[test]
fn realm_from_iss_extracts_last_segment() {
    let iss = "http://localhost:8090/realms/master";
    assert_eq!(
        realm_from_iss(iss),
        Some("master".to_string()),
        "realm should be the last path segment of iss={iss}"
    );
}

#[test]
fn realm_from_iss_ignores_trailing_slash() {
    let iss = "http://localhost:8090/realms/my-realm/";
    assert_eq!(
        realm_from_iss(iss),
        Some("my-realm".to_string()),
        "trailing slash should be trimmed for iss={iss}"
    );
}

#[test]
fn realm_from_iss_without_slash_returns_input() {
    let iss = "standalone";
    assert_eq!(
        realm_from_iss(iss),
        Some("standalone".to_string()),
        "an iss without '/' should yield itself, iss={iss}"
    );
}

/// Port 1 is reserved, so nothing ever listens on it: connecting fails the way
/// an authentication server that is not up yet does.
const UNREACHABLE_JWKS: &str = "http://127.0.0.1:1/protocol/openid-connect/certs";

/// `{"alg":"RS256","kid":"unknown-kid"}`, an empty payload and a stub signature:
/// enough for `decode_header` to succeed so validation reaches the JWKS.
const JWT_SHAPED_TOKEN: &str = "eyJhbGciOiJSUzI1NiIsImtpZCI6InVua25vd24ta2lkIn0.e30.c2ln";

#[tokio::test]
async fn new_tolerates_an_unreachable_provider() {
    let validator =
        JwtValidator::new(UNREACHABLE_JWKS, vec!["app".to_string()]).await;

    let keys = validator.keys.read().await;
    assert!(
        keys.is_none(),
        "no keys should be held when {UNREACHABLE_JWKS} is unreachable, got {keys:?}"
    );
}

#[tokio::test]
async fn validate_retries_the_fetch_and_reports_the_provider_error() {
    let validator =
        JwtValidator::new(UNREACHABLE_JWKS, vec!["app".to_string()]).await;

    let error = validator
        .validate(JWT_SHAPED_TOKEN)
        .await
        .expect_err("validation cannot succeed while the provider is unreachable");

    assert!(
        matches!(*error, AuthenticatorError::RequestError(_)),
        "validate should retry the JWKS fetch and surface its failure, got {error}"
    );
}
