# Authentication — Configuration

All defaults target a local `docker compose up` setup, so the flow works with **no extra
configuration**. Override these for other environments.

## Backend

Defined in [`crates/config/src/cli.rs`](../../crates/config/src/cli.rs) /
[`defaults.rs`](../../crates/config/src/defaults.rs). Each is available as a CLI flag and an
environment variable.

### OIDC / Backend-for-Frontend

| Env var | CLI flag | Default | Purpose |
|---------|----------|---------|---------|
| `OIDC_ISSUER_URL` | `--oidc-issuer-url` | `http://localhost:8090/realms/app` | Realm base URL; the auth / token / logout / userinfo endpoints are derived from it. |
| `OIDC_CLIENT_ID` | `--oidc-client-id` | `webapp` | Confidential client used by the BFF. |
| `OIDC_CLIENT_SECRET` | `--oidc-client-secret` | `webapp-secret` | Client secret. **Override in production.** |
| `OIDC_REDIRECT_URL` | `--oidc-redirect-url` | `http://localhost:8080/api/auth/callback` | Must be registered as a redirect URI on the `webapp` client. |
| `FRONTEND_URL` | `--frontend-url` | `http://localhost:3000` | Where the browser is sent after login; also the allowed CORS origin. |
| `COOKIE_SECURE` | `--cookie-secure` | `false` | Set `true` behind HTTPS so cookies carry `Secure`. |

### JWT validation (resource server)

| Env var | CLI flag | Default | Purpose |
|---------|----------|---------|---------|
| `AUTHENTICATOR_PROVIDER_URL` | `--authenticator-provider-url` | `http://localhost:8090/realms/app/protocol/openid-connect/certs` | JWKS endpoint used to validate token signatures. |
| `AUTHENTICATOR_AUDIENCES` | `--authenticator-audiences` | `backend` | Comma-separated audiences the access token must contain. |

### Related

| Env var | Default | Purpose |
|---------|---------|---------|
| `REDIS_URL` | `redis://127.0.0.1:6379` | Stores the PKCE verifier + CSRF state between `/auth/login` and `/auth/callback`. |

> Keep `OIDC_ISSUER_URL` and `AUTHENTICATOR_PROVIDER_URL` on the same realm, and make sure the
> `webapp` client's audience mapper emits an audience listed in `AUTHENTICATOR_AUDIENCES`.

## Frontend

Defined in [`frontend/src/config/env.ts`](../../frontend/src/config/env.ts) (Vite `VITE_APP_`
prefix). The frontend needs **no** Keycloak settings — the BFF hides them.

| Env var | Default | Purpose |
|---------|---------|---------|
| `VITE_APP_API_URL` | — (required) | Backend base URL, e.g. `http://localhost:8080/api`. Login redirects and all API calls are built from it. |
| `VITE_APP_ENABLE_API_MOCKING` | `false` | `true` serves the MSW mock auth; set `false` to use the real backend. |
| `VITE_APP_URL` | `http://localhost:3000` | The app's own origin. |

## Production checklist

- Set a strong `OIDC_CLIENT_SECRET` and rotate the realm's `webapp` secret.
- `COOKIE_SECURE=true` and serve everything over HTTPS.
- Point `OIDC_ISSUER_URL`, `OIDC_REDIRECT_URL` and `FRONTEND_URL` at real hostnames, and add
  the redirect URI + web origin to the `webapp` client.
- Consider raising `accessTokenLifespan` from the 60s dev value (see [keycloak.md](./keycloak.md)).
- In the realm, set `sslRequired` away from `none` and enable email verification once SMTP is
  configured.
