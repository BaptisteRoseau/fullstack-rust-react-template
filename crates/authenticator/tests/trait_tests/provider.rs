//! The provider-side actor the trait suites drive.
//!
//! Two things the suites need cannot be expressed through the `Authenticator`
//! trait itself: acting as the end user in front of the provider's login page, and
//! minting a credential to hand back to `validate`. Both are provider-specific, so
//! they sit behind [`ProviderAgent`], every backend implements it for its own
//! fixture, and the trait tests stay backend-agnostic.

use async_trait::async_trait;

/// The query parameters the provider hands back to the redirect URI.
pub struct CallbackParams {
    pub code: String,
    pub state: String,
}

#[async_trait]
pub trait ProviderAgent {
    /// Acts as the end user at the provider's login page and returns the `code`
    /// and `state` the provider redirects back to the callback URL with.
    async fn login(&self, authorize_url: &str) -> CallbackParams;

    /// A freshly issued access token for the credentials realm.
    async fn issue_token(&self) -> String;
}
