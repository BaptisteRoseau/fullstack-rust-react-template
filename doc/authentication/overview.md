# Authentication — Overview

## Components

```txt
┌────────────┐        ┌─────────────────────────┐        ┌────────────┐
│  Browser   │  XHR   │   Rust backend (BFF)     │  HTTP  │  Keycloak  │
│ (React SPA)│ ◀────▶ │  /api/auth/* + /api/...  │ ◀────▶ │  realm app │
└────────────┘ cookies└─────────────────────────┘        └────────────┘
       ▲  top-level redirects (login / callback)               ▲
       └───────────────────────────────────────────────────────┘
```

- **React SPA** — renders the app, reads the current user from `/api/auth/me`, and starts
  the login flow by navigating the browser to the backend. It never sees a token.
- **Rust backend (BFF)** — owns the OAuth client. It builds the Keycloak redirect, handles
  the callback, exchanges the authorization code, and stores tokens in httpOnly cookies. It
  also keeps acting as a stateless resource server, validating the access-token JWT on every
  API call.
- **Keycloak** — the identity provider. It hosts the login and registration pages and issues
  the tokens.

## The flow

```mermaid
sequenceDiagram
    participant B as Browser (SPA)
    participant API as Backend (BFF)
    participant KC as Keycloak

    Note over B: User hits a protected route, no session
    B->>API: GET /api/auth/login?redirect=/app/...
    API->>API: generate PKCE verifier + CSRF state,<br/>store in Redis keyed by state
    API-->>B: 303 redirect to Keycloak authorize URL
    B->>KC: GET /authorize (or /registrations)
    KC-->>B: hosted login / registration page
    B->>KC: submit credentials / register
    KC-->>B: 303 redirect to /api/auth/callback?code&state
    B->>API: GET /api/auth/callback?code&state
    API->>API: look up + delete state from Redis
    API->>KC: POST /token (code + PKCE verifier + client secret)
    KC-->>API: access_token + refresh_token
    API-->>B: Set-Cookie access_token, refresh_token (HttpOnly);<br/>303 redirect to the frontend
    B->>API: GET /api/auth/me (cookie sent automatically)
    API->>KC: GET /userinfo (Bearer access_token)
    KC-->>API: profile claims
    API-->>B: { id, firstName, lastName, email, ... }
    Note over B: App renders

    Note over B,KC: ...later, the access token expires...
    B->>API: GET /api/some-endpoint
    API-->>B: 401 TokenExpired
    B->>API: POST /api/auth/refresh (refresh cookie)
    API->>KC: POST /token (grant_type=refresh_token)
    KC-->>API: new access_token + refresh_token
    API-->>B: Set-Cookie (refreshed); 200
    B->>API: GET /api/some-endpoint (replayed)
    API-->>B: 200 OK
```

## Endpoints at a glance

All endpoints are served under `/api` (matching the frontend's `VITE_APP_API_URL`).

| Method & path | Purpose |
| --- | --- |
| `GET /api/auth/login` | Start the flow; `?screen=register` for the registration page, `?redirect=/path` to return to a specific route. |
| `GET /api/auth/callback` | OAuth callback; exchanges the code and sets the cookies. |
| `POST /api/auth/refresh` | Mint a fresh access token from the refresh cookie. |
| `POST /api/auth/logout` | Revoke the session at Keycloak and clear the cookies. |
| `GET /api/auth/me` | Return the current user's profile (401 when logged out). |

## Security notes

- **httpOnly cookies** keep the tokens out of reach of any script (XSS cannot exfiltrate
  them). The SPA relies on the browser attaching them automatically (`withCredentials`).
- **PKCE (S256)** protects the authorization code exchange; the verifier never leaves the
  backend.
- **CSRF on the OAuth flow**: the `state` value is generated server-side and stored in Redis;
  the callback only proceeds if it matches.
- **CORS** reflects the configured frontend origin and allows credentials (a wildcard origin
  is rejected by browsers on credentialed requests).
- `SameSite=Lax` cookies are sent on the top-level callback navigation and on same-site XHR.
  Set `COOKIE_SECURE=true` behind HTTPS in production.
