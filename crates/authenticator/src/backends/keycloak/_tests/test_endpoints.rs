use super::*;

#[test]
fn from_issuer_derives_endpoints() {
    let issuer = "http://localhost:8090/realms/app";
    let endpoints = Endpoints::from_issuer(issuer);

    assert_eq!(
        endpoints.authorize,
        "http://localhost:8090/realms/app/protocol/openid-connect/auth",
        "authorize endpoint mismatch, got={}",
        endpoints.authorize
    );
    assert_eq!(
        endpoints.registrations,
        "http://localhost:8090/realms/app/protocol/openid-connect/registrations",
        "registrations endpoint mismatch, got={}",
        endpoints.registrations
    );
    assert_eq!(
        endpoints.token, "http://localhost:8090/realms/app/protocol/openid-connect/token",
        "token endpoint mismatch, got={}",
        endpoints.token
    );
    assert_eq!(
        endpoints.logout,
        "http://localhost:8090/realms/app/protocol/openid-connect/logout",
        "logout endpoint mismatch, got={}",
        endpoints.logout
    );
    assert_eq!(
        endpoints.userinfo,
        "http://localhost:8090/realms/app/protocol/openid-connect/userinfo",
        "userinfo endpoint mismatch, got={}",
        endpoints.userinfo
    );
    assert_eq!(
        endpoints.jwks, "http://localhost:8090/realms/app/protocol/openid-connect/certs",
        "jwks endpoint mismatch, got={}",
        endpoints.jwks
    );
}

#[test]
fn from_issuer_trims_trailing_slash() {
    let issuer = "http://localhost:8090/realms/app/";
    let endpoints = Endpoints::from_issuer(issuer);

    assert_eq!(
        endpoints.authorize,
        "http://localhost:8090/realms/app/protocol/openid-connect/auth",
        "trailing slash should be trimmed, got={}",
        endpoints.authorize
    );
    assert_eq!(
        endpoints.jwks, "http://localhost:8090/realms/app/protocol/openid-connect/certs",
        "trailing slash should be trimmed, got={}",
        endpoints.jwks
    );
}
