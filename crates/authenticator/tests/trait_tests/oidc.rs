use uuid::Uuid;

use authenticator::error::AuthenticatorError;
use authenticator::{AuthSession, Authenticator, LoginScreen};
use test_trait::{test_trait, test_trait_suite};

use super::provider::ProviderAgent;
use super::{
    BFF_CLIENT_ID, BFF_EMAIL, BFF_FAMILY_NAME, BFF_GIVEN_NAME, BFF_REALM, BFF_USER_ID,
    BFF_USERNAME,
};

/// Backend-for-Frontend half of the Authenticator suite: the Authorization Code +
/// PKCE flow. See `credentials.rs` for the conventions every test here follows.
#[test_trait_suite]
pub mod suite {
    use super::*;

    /// Where the user asked to land once the login completes.
    const POST_LOGIN_REDIRECT: &str = "/app/dashboard";

    #[test_trait]
    async fn authorize_url_targets_the_login_screen(authenticator: &impl Authenticator) {
        let url = authenticator
            .authorize_url(LoginScreen::Login, Some(POST_LOGIN_REDIRECT))
            .await
            .expect("building the authorize url should succeed");

        for expected in [
            "/protocol/openid-connect/auth",
            &format!("client_id={BFF_CLIENT_ID}"),
            "code_challenge_method=S256",
            "code_challenge=",
            "state=",
            "redirect_uri=",
        ] {
            assert!(
                url.contains(expected),
                "the authorize url should carry {expected:?}, got={url}"
            );
        }
    }

    #[test_trait]
    async fn authorize_url_targets_the_registration_screen(
        authenticator: &impl Authenticator,
    ) {
        let url = authenticator
            .authorize_url(LoginScreen::Register, None)
            .await
            .expect("building the registration url should succeed");

        assert!(
            url.contains("/protocol/openid-connect/registrations"),
            "the register screen should target the registrations endpoint, got={url}"
        );
        // The PKCE and CSRF parameters must survive the endpoint swap, otherwise the
        // callback of a user who registered instead of logging in cannot complete.
        for expected in ["code_challenge=", "state=", "redirect_uri="] {
            assert!(
                url.contains(expected),
                "the registration url should carry {expected:?}, got={url}"
            );
        }
    }

    #[test_trait]
    async fn exchange_code_returns_tokens_and_redirect(
        authenticator: &impl Authenticator,
        agent: &impl ProviderAgent,
    ) {
        let session = log_in(authenticator, agent, Some(POST_LOGIN_REDIRECT)).await;

        assert!(
            !session.tokens.access_token.is_empty(),
            "the code exchange should return an access token, got={:?}",
            session.tokens
        );
        assert!(
            session.tokens.refresh_token.is_some(),
            "the code exchange should return a refresh token, got={:?}",
            session.tokens
        );
        assert_eq!(
            session.redirect.as_deref(),
            Some(POST_LOGIN_REDIRECT),
            "the post-login redirect stored at login time should come back: got={:?}, want={POST_LOGIN_REDIRECT:?}",
            session.redirect
        );
    }

    #[test_trait]
    async fn exchange_code_issues_a_token_that_validates(
        authenticator: &impl Authenticator,
        agent: &impl ProviderAgent,
    ) {
        let session = log_in(authenticator, agent, None).await;

        let user = authenticator
            .validate(&session.tokens.access_token)
            .await
            .expect("the access token from the code exchange should validate");

        let want_id =
            Uuid::parse_str(BFF_USER_ID).expect("BFF_USER_ID must be a valid uuid");
        assert_eq!(
            user.id, want_id,
            "the validated user should be the one who logged in: got={}, want={want_id}",
            user.id
        );
        assert_eq!(
            user.realm, BFF_REALM,
            "the realm should come from the iss claim: got={}, want={BFF_REALM}",
            user.realm
        );
    }

    #[test_trait]
    async fn exchange_code_rejects_an_unknown_state(authenticator: &impl Authenticator) {
        let result = authenticator
            .exchange_code("an-authorization-code", "a-state-that-was-never-issued")
            .await;

        let is_invalid_state = matches!(
            result.as_ref().map_err(|error| error.as_ref()),
            Err(AuthenticatorError::InvalidState)
        );
        assert!(
            is_invalid_state,
            "a state the backend never issued must be rejected as InvalidState, got={result:?}"
        );
    }

    #[test_trait]
    async fn exchange_code_rejects_a_replayed_state(
        authenticator: &impl Authenticator,
        agent: &impl ProviderAgent,
    ) {
        let url = authorize_url(authenticator, None).await;
        let callback = agent.login(&url).await;

        authenticator
            .exchange_code(&callback.code, &callback.state)
            .await
            .expect("the first code exchange should succeed");

        let result = authenticator
            .exchange_code(&callback.code, &callback.state)
            .await;

        let is_invalid_state = matches!(
            result.as_ref().map_err(|error| error.as_ref()),
            Err(AuthenticatorError::InvalidState)
        );
        assert!(
            is_invalid_state,
            "the login state must be consumed by the first exchange, got={result:?}"
        );
    }

    #[test_trait]
    async fn userinfo_returns_the_identity_claims(
        authenticator: &impl Authenticator,
        agent: &impl ProviderAgent,
    ) {
        let session = log_in(authenticator, agent, None).await;

        let info = authenticator
            .userinfo(&session.tokens.access_token)
            .await
            .expect("userinfo should answer for a freshly issued access token");

        let want_id =
            Uuid::parse_str(BFF_USER_ID).expect("BFF_USER_ID must be a valid uuid");
        assert_eq!(
            info.sub, want_id,
            "sub should identify the user who logged in: got={}, want={want_id}",
            info.sub
        );
        assert_eq!(
            info.preferred_username, BFF_USERNAME,
            "preferred_username mismatch: got={}, want={BFF_USERNAME}",
            info.preferred_username
        );
        assert_eq!(
            info.given_name, BFF_GIVEN_NAME,
            "given_name mismatch: got={}, want={BFF_GIVEN_NAME}",
            info.given_name
        );
        assert_eq!(
            info.family_name, BFF_FAMILY_NAME,
            "family_name mismatch: got={}, want={BFF_FAMILY_NAME}",
            info.family_name
        );
        assert_eq!(
            info.email, BFF_EMAIL,
            "email mismatch: got={}, want={BFF_EMAIL}",
            info.email
        );
    }

    #[test_trait]
    async fn refresh_tokens_issues_a_new_pair(
        authenticator: &impl Authenticator,
        agent: &impl ProviderAgent,
    ) {
        let session = log_in(authenticator, agent, None).await;
        let refresh_token = refresh_token(&session);

        let refreshed = authenticator
            .refresh_tokens(&refresh_token)
            .await
            .expect("a valid refresh token should yield a new token pair");

        assert!(
            !refreshed.access_token.is_empty(),
            "the refresh should return an access token, got={refreshed:?}"
        );
        assert_ne!(
            refreshed.access_token, session.tokens.access_token,
            "the refresh should mint a different access token, got the same one back"
        );
        assert!(
            refreshed.refresh_token.is_some(),
            "the refresh should return a refresh token, got={refreshed:?}"
        );
    }

    #[test_trait]
    async fn logout_revokes_the_session(
        authenticator: &impl Authenticator,
        agent: &impl ProviderAgent,
    ) {
        let session = log_in(authenticator, agent, None).await;
        let refresh_token = refresh_token(&session);

        authenticator
            .logout(&refresh_token)
            .await
            .expect("revoking a live session should succeed");

        let refreshed = authenticator.refresh_tokens(&refresh_token).await;
        assert!(
            refreshed.is_err(),
            "the refresh token must not survive the logout, got={refreshed:?}"
        );

        // The access token is still cryptographically valid until it expires, but the
        // provider no longer backs it with a session, which is what tells the API to
        // answer 401 rather than 500.
        let info = authenticator.userinfo(&session.tokens.access_token).await;
        let is_rejected = matches!(
            info.as_ref().map_err(|error| error.as_ref()),
            Err(AuthenticatorError::OidcRejected)
        );
        assert!(
            is_rejected,
            "userinfo on a revoked session should report OidcRejected, got={info:?}"
        );
    }

    /// Runs a full browser login and returns the resulting session.
    async fn log_in(
        authenticator: &impl Authenticator,
        agent: &impl ProviderAgent,
        redirect: Option<&str>,
    ) -> AuthSession {
        let url = authorize_url(authenticator, redirect).await;
        let callback = agent.login(&url).await;

        authenticator
            .exchange_code(&callback.code, &callback.state)
            .await
            .expect("exchanging a freshly issued authorization code should succeed")
    }

    async fn authorize_url(
        authenticator: &impl Authenticator,
        redirect: Option<&str>,
    ) -> String {
        authenticator
            .authorize_url(LoginScreen::Login, redirect)
            .await
            .expect("building the authorize url should succeed")
    }

    fn refresh_token(session: &AuthSession) -> String {
        session
            .tokens
            .refresh_token
            .clone()
            .expect("the code exchange should have returned a refresh token")
    }
}
