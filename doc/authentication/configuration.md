# Authentication — Configuration

Every default targets a local `docker compose up` setup, so the flow works with **no extra
configuration**. Override these for any other environment.

Current values are in [`crates/config/src/defaults.rs`](../../crates/config/src/defaults.rs). They
are not repeated here, because they change.

## Backend

Each setting is both a CLI flag and an environment variable. See
[`crates/config`](../../crates/config/README.md) and Skill(backend-config-entry).

### Authenticator

One section drives both roles: validating the access tokens callers present, and running the
Backend-for-Frontend login flow. Every provider endpoint — JWKS, authorize, token, logout,
userinfo — is derived from the issuer URL, so the two halves cannot drift onto different realms.

| Env var | Purpose |
| --- | --- |
| `AUTHENTICATOR_ISSUER_URL` | Realm base URL. Every other provider endpoint is derived from it |
| `AUTHENTICATOR_AUDIENCES` | Comma-separated audiences the access token must contain |
| `AUTHENTICATOR_CLIENT_ID` | The confidential client the backend uses |
| `AUTHENTICATOR_CLIENT_SECRET` | Client secret. **Always override in production** |
| `AUTHENTICATOR_REDIRECT_URL` | The callback. Must also be registered as a redirect URI on the client |

The client's audience mapper must emit an audience listed in `AUTHENTICATOR_AUDIENCES`, or every
token is rejected.

### API

| Env var | Purpose |
| --- | --- |
| `FRONTEND_URL` | Where the browser is sent after login. Also the single allowed CORS origin |
| `COOKIE_SECURE` | `true` behind HTTPS, so cookies carry `Secure` |

### Related

| Env var | Purpose |
| --- | --- |
| `REDIS_URL` | Holds the PKCE verifier and CSRF state between `/auth/login` and `/auth/callback` |

## Frontend

Validated by [`frontend/src/config/env.ts`](../../frontend/src/config/env.ts). Only variables
prefixed `VITE_APP_` reach the browser.

| Env var | Purpose |
| --- | --- |
| `VITE_APP_API_URL` | The backend **origin**, with no trailing path. The `/api` prefix is added by the client |
| `VITE_APP_ENABLE_API_MOCKING` | `true` starts the MSW worker instead of calling the backend |
| `VITE_APP_URL` | The app's own origin |

## Production checklist

- Set a strong `AUTHENTICATOR_CLIENT_SECRET`, and rotate the one shipped in the dev realm.
- Set `COOKIE_SECURE=true` and serve everything over HTTPS.
- Point `AUTHENTICATOR_ISSUER_URL`, `AUTHENTICATOR_REDIRECT_URL` and `FRONTEND_URL` at real
  hostnames, and register the redirect URI and web origin on the client.
- Raise the access-token lifespan above its development value — see [keycloak.md](./keycloak.md).
- In the realm, set `sslRequired` away from `none`, and enable email verification once SMTP works.
