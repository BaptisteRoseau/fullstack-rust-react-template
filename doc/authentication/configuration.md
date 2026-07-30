# Authentication — Configuration

All defaults target a local `docker compose up` setup, so the flow works with **no extra
configuration**. Override these for other environments.

## Backend

Defined in [`crates/config/src/cli.rs`](../../crates/config/src/cli.rs) /
[`defaults.rs`](../../crates/config/src/defaults.rs). Each is available as a CLI flag and an
environment variable.

### Authenticator

One section drives both roles: validating the access tokens callers present, and running the
Backend-for-Frontend login flow. Every provider endpoint — JWKS, authorize, token, logout,
userinfo — is derived from the issuer URL, so the two can no longer drift onto different realms.

| Env var | CLI flag | Default | Purpose |
|---------|----------|---------|---------|
| `AUTHENTICATOR_ISSUER_URL` | `--authenticator-issuer-url` | `http://localhost:8090/realms/app` | Realm base URL. |
| `AUTHENTICATOR_AUDIENCES` | `--authenticator-audiences` | `backend` | Comma-separated audiences the access token must contain. |
| `AUTHENTICATOR_CLIENT_ID` | `--authenticator-client-id` | `webapp` | Confidential client used by the BFF. |
| `AUTHENTICATOR_CLIENT_SECRET` | `--authenticator-client-secret` | `webapp-secret` | Client secret. **Override in production.** |
| `AUTHENTICATOR_REDIRECT_URL` | `--authenticator-redirect-url` | `http://localhost:8080/api/auth/callback` | Must be registered as a redirect URI on the `webapp` client. |

### API

| Env var | CLI flag | Default | Purpose |
|---------|----------|---------|---------|
| `FRONTEND_URL` | `--frontend-url` | `http://localhost:3000` | Where the browser is sent after login; also the allowed CORS origin. |
| `COOKIE_SECURE` | `--cookie-secure` | `false` | Set `true` behind HTTPS so cookies carry `Secure`. |

### Related

| Env var | Default | Purpose |
|---------|---------|---------|
| `REDIS_URL` | `redis://127.0.0.1:6379` | Stores the PKCE verifier + CSRF state between `/auth/login` and `/auth/callback`. |

> Make sure the `webapp` client's audience mapper emits an audience listed in
> `AUTHENTICATOR_AUDIENCES`.

## Frontend

Defined in [`frontend/src/config/env.ts`](../../frontend/src/config/env.ts) (Vite `VITE_APP_`
prefix). The frontend needs **no** Keycloak settings — the BFF hides them.

| Env var | Default | Purpose |
|---------|---------|---------|
| `VITE_APP_API_URL` | — (required) | Backend base URL, e.g. `http://localhost:8080/api`. Login redirects and all API calls are built from it. |
| `VITE_APP_ENABLE_API_MOCKING` | `false` | `true` serves the MSW mock auth; set `false` to use the real backend. |
| `VITE_APP_URL` | `http://localhost:3000` | The app's own origin. |

## Production checklist

- Set a strong `AUTHENTICATOR_CLIENT_SECRET` and rotate the realm's `webapp` secret.
- `COOKIE_SECURE=true` and serve everything over HTTPS.
- Point `AUTHENTICATOR_ISSUER_URL`, `AUTHENTICATOR_REDIRECT_URL` and `FRONTEND_URL` at real
  hostnames, and add the redirect URI + web origin to the `webapp` client.
- Consider raising `accessTokenLifespan` from the 60s dev value (see [keycloak.md](./keycloak.md)).
- In the realm, set `sslRequired` away from `none` and enable email verification once SMTP is
  configured.
