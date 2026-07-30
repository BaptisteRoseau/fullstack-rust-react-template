use uuid::Uuid;

use authenticator::Authenticator;
use test_utils::{trait_test, trait_test_suite};

use super::containers::{CREDENTIALS_REALM, CREDENTIALS_USER_ID};
use super::provider::ProviderAgent;

/// Credential validation half of the Authenticator suite.
///
/// When adding a test here:
/// - mark it `#[trait_test]`; the function name becomes the test name, and that is
///   the only place it is written
/// - take the subject as `&impl Authenticator`, plus `&impl ProviderAgent` when the
///   test needs a credential or a browser login
/// - helpers are unmarked functions, left alone by the macro
/// - Backend-for-Frontend tests live in `oidc.rs`
#[trait_test_suite]
pub mod suite {
    use super::*;

    #[trait_test]
    async fn validates_issued_jwt(
        authenticator: &impl Authenticator,
        agent: &impl ProviderAgent,
    ) {
        let token = agent.issue_token().await;

        let user = authenticator
            .validate(&token)
            .await
            .expect("a freshly issued provider token should be accepted");

        assert_eq!(
            user.realm, CREDENTIALS_REALM,
            "realm should be parsed from the iss claim: got={}, want={CREDENTIALS_REALM}",
            user.realm
        );
        let want_id = Uuid::parse_str(CREDENTIALS_USER_ID)
            .expect("CREDENTIALS_USER_ID must be a valid uuid");
        assert_eq!(
            user.id, want_id,
            "user id should match the sub claim: got={}, want={want_id}",
            user.id
        );
    }

    #[trait_test]
    async fn rejects_garbage_jwt(authenticator: &impl Authenticator) {
        let result = authenticator.validate("aaaa.bbbb.cccc").await;
        assert!(
            result.is_err(),
            "a structurally invalid jwt should be rejected, got={result:?}"
        );
    }

    #[trait_test]
    async fn rejects_tampered_jwt(
        authenticator: &impl Authenticator,
        agent: &impl ProviderAgent,
    ) {
        let tampered = tamper_signature(&agent.issue_token().await);

        let result = authenticator.validate(&tampered).await;
        assert!(
            result.is_err(),
            "a token with an altered signature should be rejected, got={result:?}"
        );
    }

    #[trait_test]
    async fn rejects_unknown_api_key(authenticator: &impl Authenticator) {
        // No dots => treated as an API key; the empty database reports it missing.
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
}
