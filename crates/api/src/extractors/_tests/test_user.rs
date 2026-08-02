use super::*;
use crate::error::ApiErrorResponse;
use axum::http::StatusCode;

/// Status the API would answer for a token the authenticator rejected.
fn status_for(error: AuthenticatorError) -> StatusCode {
    ApiErrorResponse::from(credential_error(Box::new(error))).status_code
}

#[test]
fn unusable_credentials_answer_unauthorized() {
    // Each of these used to fall through to a 500. They all describe a token
    // the caller sent, so they must be a 401 instead.
    let unusable = vec![
        AuthenticatorError::Message("No matching key found in JWKS".into()),
        AuthenticatorError::Message("No 'kid' in token header".into()),
        AuthenticatorError::InvalidRealm("http://provider/realms/other".into()),
        AuthenticatorError::AuthenticationFailure,
    ];

    for error in unusable {
        let display = error.to_string();
        let status = status_for(error);
        assert_eq!(
            status,
            StatusCode::UNAUTHORIZED,
            "an unusable credential must answer 401, got {status} for {display:?}"
        );
    }
}

#[test]
fn malformed_token_answers_unauthorized() {
    // A token that is not even decodable is caller input, not a server fault.
    let error = jsonwebtoken::decode_header("not-a-jwt")
        .expect_err("a malformed token must fail to decode");
    let kind = format!("{:?}", error.kind());
    let status = status_for(AuthenticatorError::JwtError(error));
    assert_eq!(
        status,
        StatusCode::UNAUTHORIZED,
        "a malformed token must answer 401, got {status} for kind {kind}"
    );
}

#[test]
fn expired_token_keeps_its_own_unauthorized() {
    let jwt_error = jsonwebtoken::errors::Error::from(
        jsonwebtoken::errors::ErrorKind::ExpiredSignature,
    );
    let kind = format!("{:?}", jwt_error.kind());
    let status = status_for(AuthenticatorError::JwtError(jwt_error));
    assert_eq!(
        status,
        StatusCode::UNAUTHORIZED,
        "an expired token must answer 401 so the frontend refreshes, got {status} for kind {kind}"
    );
}

#[test]
fn server_faults_stay_server_errors() {
    // The caller's token is irrelevant when we cannot verify anything at all.
    let status = status_for(AuthenticatorError::NoJwk);
    assert_eq!(
        status,
        StatusCode::INTERNAL_SERVER_ERROR,
        "missing JWKS is a server fault and must stay a 500, got {status}"
    );
}
