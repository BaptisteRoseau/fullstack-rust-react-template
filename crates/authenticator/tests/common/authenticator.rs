use uuid::Uuid;

use authenticator::Authenticator;

use super::containers::{KeycloakFixture, REALM, USER_ID};

// When adding a new test here:
// - helpers are regular private functions
// - tests signature is `pub async fn assert_<my test>(fixture: &KeycloakFixture)`
// - new tests should be added in the `authenticator_trait_tests` macro

/// Set of integration tests for the Authenticator trait, backed by a live
/// Keycloak container. Returns a `Vec<Trial>` for use with `libtest-mimic`.
/// The caller must have `mod common;` declared beforehand.
macro_rules! authenticator_trait_tests {
    ($fixture:expr, $rt:expr) => {{
        use common::authenticator::*;
        use libtest_mimic::Trial;
        use std::sync::Arc;

        let rt: Arc<tokio::runtime::Runtime> = $rt;
        let fixture: Arc<KeycloakFixture> = $fixture;

        vec![
            {
                let rt = rt.clone();
                let fixture = fixture.clone();
                Trial::test("validates_valid_jwt", move || {
                    rt.block_on(assert_validates_valid_jwt(&fixture));
                    Ok(())
                })
            },
            {
                let rt = rt.clone();
                let fixture = fixture.clone();
                Trial::test("rejects_garbage_jwt", move || {
                    rt.block_on(assert_rejects_garbage_jwt(&fixture));
                    Ok(())
                })
            },
            {
                let rt = rt.clone();
                let fixture = fixture.clone();
                Trial::test("rejects_tampered_jwt", move || {
                    rt.block_on(assert_rejects_tampered_jwt(&fixture));
                    Ok(())
                })
            },
            {
                let rt = rt.clone();
                let fixture = fixture.clone();
                Trial::test("rejects_unknown_api_key", move || {
                    rt.block_on(assert_rejects_unknown_api_key(&fixture));
                    Ok(())
                })
            },
        ]
    }};
}

pub async fn assert_validates_valid_jwt(fixture: &KeycloakFixture) {
    let authenticator = fixture.authenticator().await;
    let token = fixture.fetch_token().await;

    let user = authenticator
        .validate(&token)
        .await
        .expect("a freshly issued keycloak token should be accepted");

    assert_eq!(
        user.realm, REALM,
        "realm should be parsed from the iss claim: got={}, want={REALM}",
        user.realm
    );
    let want_id =
        Uuid::parse_str(USER_ID).expect("USER_ID constant must be a valid uuid");
    assert_eq!(
        user.id, want_id,
        "user id should match the sub claim: got={}, want={want_id}",
        user.id
    );
}

pub async fn assert_rejects_garbage_jwt(fixture: &KeycloakFixture) {
    let authenticator = fixture.authenticator().await;

    let result = authenticator.validate("aaaa.bbbb.cccc").await;
    assert!(
        result.is_err(),
        "a structurally invalid jwt should be rejected, got={result:?}"
    );
}

pub async fn assert_rejects_tampered_jwt(fixture: &KeycloakFixture) {
    let authenticator = fixture.authenticator().await;
    let token = fixture.fetch_token().await;
    let tampered = tamper_signature(&token);

    let result = authenticator.validate(&tampered).await;
    assert!(
        result.is_err(),
        "a token with an altered signature should be rejected, got={result:?}"
    );
}

pub async fn assert_rejects_unknown_api_key(fixture: &KeycloakFixture) {
    let authenticator = fixture.authenticator().await;

    // No dots => treated as an API key; the no-op database reports it missing.
    let result = authenticator.validate("plain-api-key-without-dots").await;
    assert!(
        result.is_err(),
        "an unknown api key should be rejected, got={result:?}"
    );
}

/// Flips a character in the JWT signature segment so the RS256 check fails.
fn tamper_signature(token: &str) -> String {
    let (rest, signature) = token
        .rsplit_once('.')
        .expect("a jwt must contain a signature segment");
    let mut chars: Vec<char> = signature.chars().collect();
    assert!(
        !chars.is_empty(),
        "jwt signature segment must not be empty, token={token}"
    );

    let middle = chars.len() / 2;
    chars[middle] = if chars[middle] == 'A' { 'B' } else { 'A' };

    format!("{rest}.{}", chars.into_iter().collect::<String>())
}
