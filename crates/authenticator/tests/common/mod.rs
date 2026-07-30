mod authenticator;
mod containers;
mod oidc;
mod provider;

test_trait::test_trait_main!(containers::KeycloakFixture);
