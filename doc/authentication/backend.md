# Authentication — Backend

The backend plays two roles:

1. **Resource server** — it validates the access-token JWT on every API request (unchanged,
   pre-existing behaviour).
2. **Backend-for-Frontend (BFF)** — it drives the OAuth flow and manages the cookie session.

All OAuth logic lives in the **`authenticator`** crate. The **`api`** crate stays thin: it
extracts input, sets/clears cookies, routes, and converts errors.

## OAuth logic — `authenticator` crate

[`crates/authenticator/src/oidc.rs`](../../crates/authenticator/src/oidc.rs) defines
`OidcClient`, built on the [`oauth2`](https://docs.rs/oauth2) v5 crate.

| Method | Responsibility |
|--------|----------------|
| `authorize_url(screen, redirect)` | Generate a PKCE challenge + CSRF state, store `{verifier, redirect}` in the shared cache keyed by the state, and return the Keycloak authorize URL. For `LoginScreen::Register`, the path is rewritten to Keycloak's `/registrations` endpoint. |
| `exchange_code(code, state)` | Look up and delete the cached state, exchange the code (with the PKCE verifier) at the token endpoint, and return the tokens plus the stored post-login redirect. |
| `refresh(refresh_token)` | Exchange a refresh token for a fresh token set. |
| `logout(refresh_token)` | Revoke the session at Keycloak's end-session endpoint. |
| `userinfo(access_token)` | Fetch the OIDC userinfo claims for `/auth/me`. |

Notes:

- The PKCE verifier and CSRF state are persisted in **Redis** (the shared `Cache`) under
  `oidc_state:{state}` with a 600s TTL. This bridges `/auth/login` and `/auth/callback` and
  doubles as CSRF protection — the callback only proceeds if the state matches.
- The HTTP client comes from `oauth2::reqwest` (the crate's pinned reqwest) so it satisfies
  the `AsyncHttpClient` bound; `redirect::Policy::none()` is set as the crate requires.
- JWT validation itself remains in
  [`crates/authenticator/src/backends/keycloak.rs`](../../crates/authenticator/src/backends/keycloak.rs):
  RS256 signature check against JWKS + audience validation, extracting `sub` and `iss`.

## HTTP endpoints — `api` crate

[`crates/api/src/endpoints/auth/`](../../crates/api/src/endpoints/auth/) — handlers in
`endpoints.rs`, request/response types in `models.rs`. They follow the project's utoipa
endpoint convention and are registered in
[`crates/api/src/routes.rs`](../../crates/api/src/routes.rs).

| Endpoint | Behaviour |
|----------|-----------|
| `GET /auth/login` | `LoginScreen` chosen from `?screen`; 303 redirect to Keycloak. |
| `GET /auth/callback` | Exchange the code, `Set-Cookie` the tokens, 303 to the frontend. Missing code (cancel) bounces back to the frontend. |
| `POST /auth/refresh` | Refresh from the cookie; `200` + new cookies, or `401` (and cleared cookies) on failure so the SPA knows to re-login. |
| `POST /auth/logout` | Best-effort revoke + clear cookies; `204`. |
| `GET /auth/me` | Requires authentication; returns the profile from `userinfo`. |

> The whole API router is nested under `/api`, so the live paths are `/api/auth/login`, etc.

### Cookies

Set in [`endpoints.rs`](../../crates/api/src/endpoints/auth/endpoints.rs) via `axum-extra`'s
`CookieJar`:

- `access_token` and `refresh_token`
- `HttpOnly`, `SameSite=Lax`, `Path=/`
- `Secure` controlled by `COOKIE_SECURE` (enable behind HTTPS)
- session cookies (no `Max-Age`): expiry is enforced by the JWT's own `exp`, which keeps the
  expired cookie available so the backend can answer `401 TokenExpired` and trigger a refresh.

The post-login redirect is resolved against the configured frontend origin and only honours
same-origin paths (starting with `/`) to avoid open redirects.

## Reading the token — extractor

[`crates/api/src/extractors/user.rs`](../../crates/api/src/extractors/user.rs) resolves the
`UserToken` from either:

1. the `Authorization: Bearer <token>` header (API keys / machine clients), or
2. the `access_token` cookie (the SPA, via the BFF).

This lets cookie-authenticated browser calls and header-based clients coexist.

## Error mapping — silent refresh hinge

[`crates/api/src/error/response.rs`](../../crates/api/src/error/response.rs) maps an **expired
token** to **`401 TokenExpired`** (previously it fell through to a `500`). This 401 is exactly
what the frontend interceptor watches for to perform a silent refresh. Other JWT failures
(invalid signature/subject/issuer) map to `403`.

## CORS

[`routes.rs`](../../crates/api/src/routes.rs) replaces the permissive CORS layer with one that
reflects the configured frontend origin and sets `allow_credentials(true)` — a wildcard origin
is rejected by browsers on credentialed (cookie) requests.

## Wiring

`OidcClient` is constructed in
[`crates/binaries/backend/src/program.rs`](../../crates/binaries/backend/src/program.rs) and
held in `AppState` alongside the existing authenticator. See
[configuration.md](./configuration.md) for the settings it reads.
