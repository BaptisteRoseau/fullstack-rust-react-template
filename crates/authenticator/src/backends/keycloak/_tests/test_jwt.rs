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
