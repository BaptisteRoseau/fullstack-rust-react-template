# Authentication — Manual Testing Guide

How to run the full register/login flow from the browser, and how to obtain a raw JWT for
`curl`-based API testing. For the design, see [overview.md](./overview.md).

## Prerequisites

- Docker + Docker Compose
- `curl` and `jq` (for the token commands)

---

## 1. Start the services

The root compose file includes the authentication stack, so a plain `up` is enough. The `app`
realm (clients, self-service registration, the `testuser` account is **not** created here —
you register one yourself) is imported automatically on first boot.

```bash
docker compose up -d
```

Wait ~15 seconds for Keycloak to finish booting, then verify it is up and the realm imported:

```bash
curl -s http://localhost:8090/health/ready | jq .
# -> { "status": "UP" }

curl -s http://localhost:8090/realms/app/.well-known/openid-configuration | jq .issuer
# -> "http://localhost:8090/realms/app"
```

> No manual realm/client setup is required — it is provisioned from
> `infrastructure/docker-compose/keycloak/import/realm-export.json`. The admin console is at
> **http://localhost:8090** (`admin` / `admin`) if you want to inspect it.

---

## 2. Run the application

```bash
# Backend (defaults already point at the local Keycloak realm `app`)
cargo run -p backend

# Frontend against the real backend (mocks off), in another terminal
cd frontend && VITE_APP_ENABLE_API_MOCKING=false bun run dev
```

---

## 3. Register and log in from the browser

1. Open the app (http://localhost:3000) and navigate to a protected route.
2. You are redirected to the login screen → **Create an account** (or **Continue to sign in**).
   This sends you to Keycloak's hosted page.
3. Register a new user (no email verification is required) or log in.
4. Keycloak redirects back through `/api/auth/callback`; the backend sets the httpOnly cookies
   and returns you to the app, now authenticated.

To watch the **silent refresh**: stay on the app for ~60s (the access-token lifespan) and keep
using it. In the browser dev-tools Network tab you will see a `401`, an automatic
`POST /api/auth/refresh`, and the original request replayed — with no visible re-login.

Log out from the app to revoke the session and clear the cookies.

---

## 4. Obtain a raw JWT for `curl` (direct access grant)

The browser flow stores tokens in httpOnly cookies, which are awkward to use from `curl`. For
scripted API testing, use the public `backend` client's direct access grant with a user you
created in step 3:

```bash
TOKEN_RESPONSE=$(curl -s -X POST \
  http://localhost:8090/realms/app/protocol/openid-connect/token \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "grant_type=password" \
  -d "client_id=backend" \
  -d "scope=openid" \
  -d "username=<your-username>" \
  -d "password=<your-password>")

ACCESS_TOKEN=$(echo "$TOKEN_RESPONSE" | jq -r .access_token)
```

Inspect the payload (the `sub` is the user UUID surfaced as `UserToken.id`):

```bash
echo "$ACCESS_TOKEN" | cut -d. -f2 | base64 -d 2>/dev/null | jq .
```

---

## 5. Call the API

The backend runs on port **8080** and every route is under `/api`.

```bash
# Health check (no auth)
curl -s http://localhost:8080/api/ping
```

Protected API routes accept the token either as an `Authorization: Bearer` header (used by API
keys and machine clients) or as the `access_token` cookie (used by the browser flow). Note that
`/api/auth/me` specifically reads the **cookie**, because it calls Keycloak's userinfo endpoint
on your behalf:

```bash
# /auth/me — cookie form
curl -s http://localhost:8080/api/auth/me \
  --cookie "access_token=$ACCESS_TOKEN" | jq .

# A header-authenticated route, e.g. creating an API key
curl -s -X POST http://localhost:8080/api/api-key \
  -H "Authorization: Bearer $ACCESS_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{ "name": "test", "permissions": [] }' | jq .
```

---

## 6. Refresh and expiry (manual)

The token response also contains a `refresh_token`:

```bash
NEW_TOKEN=$(curl -s -X POST \
  http://localhost:8090/realms/app/protocol/openid-connect/token \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "grant_type=refresh_token" \
  -d "client_id=backend" \
  -d "refresh_token=$(echo "$TOKEN_RESPONSE" | jq -r .refresh_token)" \
  | jq -r .access_token)
```

The application's own refresh is handled by `POST /api/auth/refresh` using the refresh cookie.

---

## 7. Troubleshooting

| Symptom | Likely cause |
|---------|-------------|
| `401 TokenExpired` on an API call | Access token expired — expected; the frontend refreshes automatically. With `curl`, fetch a new token (step 4) or refresh (step 6). |
| `401 Unauthorized` right after login | Cookies not sent — ensure the frontend uses `withCredentials` and `FRONTEND_URL` matches the browser origin (CORS). |
| Redirected to Keycloak in a loop | The `webapp` redirect URI must match `OIDC_REDIRECT_URL` exactly; check the realm client config. |
| `invalid sub UUID` | Keycloak `sub` is not a UUID — inspect the payload (step 4). |
| `No matching key found in JWKS` | Backend fetched JWKS before Keycloak was ready; restart the backend to force a refresh. |
| Realm `app` not found | The realm did not import — check `docker compose logs keycloak`; on a re-import remove the `postgres_keycloak` volume first. |
| Keycloak not reachable | `docker compose ps` — `postgres_keycloak` must be healthy before Keycloak starts. |
