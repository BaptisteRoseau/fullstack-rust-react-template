# Authentication — Manual Testing Guide

How to run the full register → login → logout → login flow end to end, either from the browser
or with `curl` alone, and how to obtain a raw JWT for API testing. For the design, see
[overview.md](./overview.md).

## Prerequisites

- Docker + Docker Compose
- `curl` and `jq`

---

## 1. Start the services

The root compose file includes the authentication stack, so a plain `up` is enough. The `app`
realm is imported automatically on first boot. No user is pre-created — you register one
yourself in step 3.

```bash
docker compose up -d
```

The flow needs **Keycloak**, **Redis** and **Mailhog**:

| Service | Port | Role in the flow |
|---------|------|------------------|
| Keycloak | 8090 | Hosts the login/registration pages, issues the tokens. |
| Redis | 6379 | Stores the PKCE verifier + CSRF state between `/auth/login` and `/auth/callback`. |
| Mailhog | 8025 (UI) | Catches the registration verification email (the realm has `verifyEmail: true`). |

> Redis is **not optional**: without it `/api/auth/login` and `/api/auth/register` fail with
> `500 UNEXPECTED`, because the login state cannot be persisted.

Wait ~15 seconds for Keycloak to boot, then verify the realm imported:

```bash
curl -s http://localhost:8090/realms/app/.well-known/openid-configuration | jq .issuer
# -> "http://localhost:8090/realms/app"
```

> Keycloak's `/health/ready` endpoint lives on the **management port 9000**, which is not
> published by the compose file — `curl http://localhost:8090/health/ready` returns 404. Use
> the well-known document above as the readiness check instead.

Check that the two application clients exist:

```bash
curl -s http://localhost:8090/realms/app/.well-known/openid-configuration >/dev/null && \
ADMIN=$(curl -s -X POST http://localhost:8090/realms/master/protocol/openid-connect/token \
  -d grant_type=password -d client_id=admin-cli -d username=admin -d password=admin | jq -r .access_token)
curl -s -H "Authorization: Bearer $ADMIN" http://localhost:8090/admin/realms/app/clients \
  | jq -r '.[] | select(.clientId=="webapp" or .clientId=="backend") | .clientId'
# -> webapp
# -> backend
```

If they are missing, the realm import did not contain them — see
[keycloak.md](./keycloak.md). The admin console is at **http://localhost:8090**
(`admin` / `admin`).

---

## 2. Run the application

```bash
# Backend (defaults already point at the local Keycloak realm `app`)
cargo run -p backend

# Frontend against the real backend (mocks off), in another terminal
cd frontend && VITE_APP_ENABLE_API_MOCKING=false bun run dev
```

The backend listens on **8080**, the frontend on **3000**, and every API route is under `/api`.

---

## 3. Register and log in from the browser

1. Open the app (http://localhost:3000) and navigate to a protected route.
2. You are redirected to the login screen → **Create an account** (or **Continue to sign in**).
   This sends you to Keycloak's hosted page.
3. Register a new user. Keycloak then asks you to confirm your address: open
   **http://localhost:8025** (Mailhog), click the link in the *Verify email* message, and the
   flow resumes automatically.
4. Keycloak redirects back through `/api/auth/callback`; the backend sets the httpOnly cookies
   and returns you to the app, now authenticated.

Log out from the app to revoke the session and clear the cookies.

---

## 4. The same flow with `curl` only

This exercises the whole Backend-for-Frontend dance without a browser. Keycloak's pages are
plain HTML forms, so `curl` can drive them as long as you keep its cookie jar.

### 4.1 Register

```bash
USERNAME=manualtest
PASSWORD='Str0ngPass!23'
EMAIL=manualtest@example.com

# a) Ask the backend where to register. It answers 303 with the Keycloak URL.
REG_URL=$(curl -s -i "http://localhost:8080/api/auth/register?redirect=/dashboard" \
  | grep -i '^location:' | sed 's/^[Ll]ocation: //' | tr -d '\r')

# b) Fetch the registration page and extract the form target (it carries the session code).
ACTION=$(curl -s -c kc.jar "$REG_URL" \
  | grep -o 'action="[^"]*"' | head -1 | sed 's/action="//; s/"$//; s/&amp;/\&/g')

# c) Submit the registration form.
curl -s -i -b kc.jar -c kc.jar -X POST "$ACTION" \
  --data-urlencode "firstName=Manual" \
  --data-urlencode "lastName=Test" \
  --data-urlencode "email=$EMAIL" \
  --data-urlencode "username=$USERNAME" \
  --data-urlencode "password=$PASSWORD" \
  --data-urlencode "password-confirm=$PASSWORD" -D reg.hdr -o /dev/null
grep -i '^location' reg.hdr
```

The `Location` tells you what happens next. With the realm's default `verifyEmail: true` it is
`.../login-actions/required-action?execution=VERIFY_EMAIL...` → continue with 4.2. If you
turned verification off, it is already
`http://localhost:8080/api/auth/callback?code=...&state=...` → jump straight to 4.3.

### 4.2 Confirm the email through Mailhog

```bash
# Follow the required-action redirect so Keycloak sends the mail.
curl -s -b kc.jar -c kc.jar "$(grep -i '^location:' reg.hdr | sed 's/^[Ll]ocation: //' | tr -d '\r')" -o /dev/null

# Pull the verification link out of the (quoted-printable) message body.
VERIFY=$(curl -s http://localhost:8025/api/v2/messages | jq -r '.items[0].Content.Body' | python3 -c "
import sys, quopri, re
body = quopri.decodestring(sys.stdin.read().encode()).decode('utf8', 'replace')
m = re.search(r'http://localhost:8090/realms/app/login-actions/action-token[^\s\"<>]+', body)
print(m.group(0) if m else '')")

# Following it marks the address verified and resumes the OAuth flow.
curl -s -i -b kc.jar -c kc.jar "$VERIFY" -D verify.hdr -o /dev/null
CALLBACK=$(curl -s -i -b kc.jar -c kc.jar \
  "$(grep -i '^location:' verify.hdr | sed 's/^[Ll]ocation: //' | tr -d '\r')" \
  | grep -i '^location:' | sed 's/^[Ll]ocation: //' | tr -d '\r')
```

### 4.3 Exchange the code for cookies

```bash
curl -s -i -c app.jar "$CALLBACK" | grep -i '^HTTP\|^location'
# HTTP/1.1 303 See Other
# location: http://localhost:3000/dashboard      <- the ?redirect= you asked for
```

`app.jar` now holds the two httpOnly cookies (`access_token`, `refresh_token`,
`SameSite=Lax`, `Path=/`; `Secure` only when `COOKIE_SECURE=true`).

```bash
curl -s -b app.jar http://localhost:8080/api/auth/me | jq .
# { "id": "...uuid...", "firstName": "Manual", "lastName": "Test", ... }
```

### 4.4 Log out

```bash
curl -s -i -b app.jar -c app.jar -X POST http://localhost:8080/api/auth/logout | grep -i '^HTTP\|^set-cookie'
# HTTP/1.1 204 No Content
# set-cookie: access_token=; Path=/; Max-Age=0; ...
# set-cookie: refresh_token=; Path=/; Max-Age=0; ...
```

Logout revokes the session **at Keycloak**, not just locally. The old tokens stop working even
though the access token has not expired yet:

```bash
curl -s -o /dev/null -w '%{http_code}\n' -H "Authorization: Bearer $OLD_ACCESS" \
  http://localhost:8090/realms/app/protocol/openid-connect/userinfo      # -> 401
curl -s -b old.jar http://localhost:8080/api/auth/me | jq -c .           # -> 401 UNAUTHORIZED
```

### 4.5 Log back in

```bash
LOGIN_URL=$(curl -s -i "http://localhost:8080/api/auth/login?redirect=/profile" \
  | grep -i '^location:' | sed 's/^[Ll]ocation: //' | tr -d '\r')

ACTION=$(curl -s -c kc2.jar "$LOGIN_URL" \
  | grep -o 'action="[^"]*"' | head -1 | sed 's/action="//; s/"$//; s/&amp;/\&/g')

CALLBACK=$(curl -s -i -b kc2.jar -c kc2.jar -X POST "$ACTION" \
  --data-urlencode "username=$USERNAME" --data-urlencode "password=$PASSWORD" \
  | grep -i '^location:' | sed 's/^[Ll]ocation: //' | tr -d '\r')

curl -s -i -c app2.jar "$CALLBACK" | grep -i '^HTTP\|^location'   # 303 -> http://localhost:3000/profile
curl -s -b app2.jar http://localhost:8080/api/auth/me | jq -c .   # 200, same "id" as before
```

The user UUID (`id`) is identical to the one from step 4.3 — the account persisted across the
logout.

---

## 5. Obtain a raw JWT for `curl` (direct access grant)

The browser flow stores tokens in httpOnly cookies, which are awkward to script. For plain API
testing, use the public `backend` client's direct access grant with the user you registered:

```bash
TOKEN_RESPONSE=$(curl -s -X POST \
  http://localhost:8090/realms/app/protocol/openid-connect/token \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "grant_type=password" \
  -d "client_id=backend" \
  -d "scope=openid" \
  -d "username=$USERNAME" \
  -d "password=$PASSWORD")

ACCESS_TOKEN=$(echo "$TOKEN_RESPONSE" | jq -r .access_token)
```

Inspect the payload (`sub` is the user UUID surfaced as `UserToken.id`, and `aud` must contain
`backend` for the API's audience check to pass):

```bash
echo "$ACCESS_TOKEN" | cut -d. -f2 | base64 -d 2>/dev/null | jq '{sub, aud, azp, preferred_username, exp}'
```

---

## 6. Call the API

```bash
# Health check (no auth)
curl -s http://localhost:8080/api/ping        # -> pong
```

Protected routes accept the token either as an `Authorization: Bearer` header (API keys and
machine clients) or as the `access_token` cookie (browser flow). `/api/auth/me` reads the
**cookie**, because it calls Keycloak's userinfo endpoint on your behalf:

```bash
curl -s http://localhost:8080/api/auth/me --cookie "access_token=$ACCESS_TOKEN" | jq .

curl -s -X POST http://localhost:8080/api/api-key \
  -H "Authorization: Bearer $ACCESS_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{ "name": "test", "permissions": [] }' | jq .
```

---

## 7. Refresh and expiry

The access token lives for **300 seconds** (`accessTokenLifespan` in the realm). The
application refreshes it through the refresh cookie:

```bash
curl -s -i -b app2.jar -c app2.jar -X POST http://localhost:8080/api/auth/refresh | grep -i '^HTTP'
# HTTP/1.1 200 OK   -- and app2.jar now holds a new access_token
```

`POST /api/auth/refresh` answers **401** when the refresh cookie is missing, expired or
revoked, which is the frontend's signal to send the user back through the login flow.

Refreshing directly against Keycloak, for comparison:

```bash
curl -s -X POST http://localhost:8090/realms/app/protocol/openid-connect/token \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "grant_type=refresh_token" \
  -d "client_id=backend" \
  -d "refresh_token=$(echo "$TOKEN_RESPONSE" | jq -r .refresh_token)" | jq -r .access_token
```

---

## 8. Expected error responses

Worth checking after any change to the auth stack:

| Request | Expected |
|---------|----------|
| `/api/auth/me` without cookie | `401 UNAUTHORIZED` |
| `/api/auth/me` with a revoked (but unexpired) token | `401 UNAUTHORIZED` |
| `/api/auth/me` with an expired token | `401 TOKEN_EXPIRED` (frontend then refreshes) |
| `/api/auth/refresh` without cookie | `401 UNAUTHORIZED` |
| `/api/auth/callback?code=..&state=bogus` | `401 UNAUTHORIZED` (CSRF state check) |
| `/api/auth/callback` with no `code` (user cancelled) | `303` back to the frontend root |
| `/api/auth/login?redirect=https://evil.com` | `303`; after login you land on the frontend root, **not** on the foreign origin |

Only same-origin paths beginning with `/` are honored as post-login redirects.

---

## 9. Troubleshooting

| Symptom | Likely cause |
|---------|-------------|
| `500 UNEXPECTED` on `/api/auth/login` or `/api/auth/register` | Redis is not reachable — the PKCE/CSRF state cannot be stored. `docker compose up -d redis`. |
| `curl` gets an empty reply and the backend logs a `CryptoProvider` panic | Two crates enabled conflicting `jsonwebtoken` crypto features. Exactly one of `rust_crypto` / `aws_lc_rs` must be on across the workspace. |
| `invalid_client` / `unauthorized_client` at the token endpoint | The `webapp` or `backend` client is missing from the realm — check step 1. |
| `401 TOKEN_EXPIRED` on an API call | Access token expired — expected; the frontend refreshes automatically. With `curl`, refresh (step 7) or fetch a new token (step 5). |
| `401 UNAUTHORIZED` right after login | Cookies not sent — ensure the frontend uses `withCredentials` and `FRONTEND_URL` matches the browser origin (CORS). |
| Stuck on Keycloak's "Verify email" page | Open Mailhog at http://localhost:8025 and follow the link, or set `verifyEmail: false` in the realm. |
| Redirected to Keycloak in a loop | The `webapp` redirect URI must match `OIDC_REDIRECT_URL` exactly; check the realm client config. |
| `invalid sub UUID` | Keycloak `sub` is not a UUID — inspect the payload (step 5). |
| `No matching key found in JWKS` | Backend fetched JWKS before Keycloak was ready; restart the backend to force a refresh. |
| Realm `app` not found | The realm did not import — check `docker compose logs keycloak`; on a re-import remove the `postgres_keycloak` volume first. |
| Keycloak not reachable | `docker compose ps` — `postgres_keycloak` must be healthy before Keycloak starts. |
