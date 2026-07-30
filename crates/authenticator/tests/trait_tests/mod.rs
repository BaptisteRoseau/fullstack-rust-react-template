//! The `Authenticator` trait suite, split in two because the trait spans two
//! provider roles: [`credentials`] covers `validate`, [`oidc`] covers the
//! Backend-for-Frontend login flow.
//!
//! The constants below are what the suites assert on, so they are the suite's
//! side of the contract: a backend fixture provisions a provider that matches
//! them, and the suites run against it unchanged.

pub mod credentials;
pub mod oidc;
pub mod provider;

/// Realm minting the tokens `validate` is handed, mirrored back as their `iss`.
pub const CREDENTIALS_REALM: &str = "test-realm";
/// The credentials realm's user, mirrored back as the JWT `sub` claim.
pub const CREDENTIALS_USER_ID: &str = "11111111-1111-1111-1111-111111111111";

/// Realm running the Authorization Code + PKCE flow.
pub const BFF_REALM: &str = "oidc-test-realm";
/// The client the login flow authenticates as; it shows up in the authorize URL.
pub const BFF_CLIENT_ID: &str = "webapp";

/// The identity the BFF realm's user logs in as, as `userinfo` should report it.
pub const BFF_USER_ID: &str = "22222222-2222-2222-2222-222222222222";
pub const BFF_USERNAME: &str = "oidcuser";
pub const BFF_EMAIL: &str = "oidcuser@example.com";
pub const BFF_GIVEN_NAME: &str = "Oidc";
pub const BFF_FAMILY_NAME: &str = "User";
