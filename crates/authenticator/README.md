# Authenticator

This is the authentication layer. It provides an interface to everything the application
needs from an identity provider, and a Keycloak backend implementing it.

The crate covers two roles that are two halves of the same relationship with the provider,
so they live behind one trait:

- **Resource server** — resolving the credential a caller presents (a provider JWT or an
  API key) into the user it identifies.
- **Backend-for-Frontend** — driving the browser's Authorization Code + PKCE login on the
  user's behalf, so the SPA never handles a token.

## The trait

`Authenticator` is the only thing `app_core` and `api` depend on. Every method takes
`&self`, so it needs no interior mutability.

| Method | Responsibility |
|--------|----------------|
| `validate` | Resolve a caller's credential into a `UserToken`. |
| `authorize_url` | Build the provider URL to send the browser to, persisting the PKCE verifier and the post-login redirect under a fresh CSRF state. |
| `exchange_code` | Trade an authorization code for an `AuthSession`, validating and consuming the CSRF state. |
| `refresh_tokens` | Trade a refresh token for a fresh pair. |
| `userinfo` | Fetch the identity claims backing the current-user endpoint. |
| `logout` | Revoke the provider-side session. |

Shared types live in `models.rs`: `UserToken`, `LoginScreen`, `AuthTokens`, `AuthSession`
and `UserInfo`. Errors are `AuthenticatorError` in `error.rs`.

## The Keycloak backend

`backends::Keycloak` implements the trait. It is split by responsibility rather than kept
as one file:

| Module | Contents |
|--------|----------|
| `backend.rs` | The `Keycloak` struct, its constructor, and the trait impl delegating to the three modules below. |
| `endpoints.rs` | `Endpoints::from_issuer` — the single place Keycloak's URL shape is encoded. |
| `jwt.rs` | RS256 validation against the realm's JWKS, plus the `iss` → realm extraction. |
| `api_key.rs` | The sha256 digest lookup, memoised in the shared cache for 300s. |
| `oidc.rs` | The Authorization Code + PKCE flow, on top of the `oauth2` crate. |

Notes worth knowing before changing anything here:

- **Every endpoint is derived from the issuer URL.** JWKS, authorize, token, logout and
  userinfo all come out of `Endpoints::from_issuer`, so they cannot drift onto different
  realms.
- **The JWKS is fetched once**, when `Keycloak::try_new` runs. A provider that is not up
  yet is not fatal: the failure is logged as a warning and the next `validate` retries the
  fetch, so the backend boots without Keycloak. A key rotation at the provider still
  breaks validation until the backend restarts, since the keys are not re-fetched once
  held.
- **The login state lives in the shared cache**, under `oidc_state:{state}` with a 600s
  TTL. It carries the PKCE verifier and the post-login redirect, and doubles as CSRF
  protection: `exchange_code` deletes it, so a state cannot be replayed.
- **`LoginScreen::Register`** swaps the authorize endpoint for Keycloak's
  `/registrations` one, keeping the query string oauth2 built. That is a Keycloak
  extension, not part of OIDC.
- **A credential is dispatched on its shape**: only a JWT contains dots, anything else is
  treated as an API key.

## Configuration

Read from `config.authenticator` (see `crates/config`): `issuer_url`, `audiences`,
`client_id`, `client_secret`, `redirect_url`. Cookie and redirect policy are *not* the
authenticator's business — they belong to the API layer and live in `config.api`.

## Testing

Unit tests sit next to the code they cover. Integration tests run against a real Keycloak
container — see [tests/README.md](./tests/README.md).

```sh
cargo test -p authenticator
```
