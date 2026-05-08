# Authentication — Manual Testing Guide

This document covers how to start Keycloak, perform the one-time realm setup, register a user, obtain a JWT, and call the API with it.

## Prerequisites

- Docker + Docker Compose
- `curl` and `jq` (for the token commands)

---

## 1. Start the services

Keycloak runs in its own compose file and must be started alongside the base services:

```bash
docker compose \
  -f infrastructure/docker-compose/docker-compose.base.yml \
  -f infrastructure/docker-compose/docker-compose.authentication.yml \
  up -d
```

Wait ~15 seconds for Keycloak to finish booting, then verify:

```bash
curl -s http://localhost:8090/health/ready | jq .
```

Expected: `{ "status": "UP" }`

---

## 2. One-time Keycloak setup (admin console)

Open **http://localhost:8090** and log in with:

| Field    | Value   |
|----------|---------|
| Username | `admin` |
| Password | `admin` |

### 2.1 Create a client

1. Go to **Clients → Create client**
2. Set **Client ID** to `backend` (must match `DEFAULT_AUTHENTICATOR_AUDIENCES` in config)
3. Leave **Client authentication** OFF (public client)
4. Enable **Direct access grants** (allows username/password token exchange — convenient for testing)
5. Save

### 2.2 Enable user self-registration (optional)

1. Go to **Realm settings → Login**
2. Toggle **User registration** ON
3. Save

---

## 3. Create a test user

Either via the admin console or the CLI:

**Admin console:** Users → Create user → fill in Username + email → set a password under the Credentials tab (toggle Temporary OFF).

**CLI equivalent:**

```bash
# 1. Get an admin token
ADMIN_TOKEN=$(curl -s -X POST http://localhost:8090/realms/master/protocol/openid-connect/token \
  -d "grant_type=password" \
  -d "client_id=admin-cli" \
  -d "username=admin" \
  -d "password=admin" \
  | jq -r .access_token)

# 2. Create the user
curl -s -X POST http://localhost:8090/admin/realms/master/users \
  -H "Authorization: Bearer $ADMIN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "email": "testuser@example.com",
    "enabled": true,
    "credentials": [{ "type": "password", "value": "password", "temporary": false }]
  }'
```

---

## 4. Log in and obtain a JWT

```bash
TOKEN_RESPONSE=$(curl -s -X POST \
  http://localhost:8090/realms/master/protocol/openid-connect/token \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "grant_type=password" \
  -d "client_id=backend" \
  -d "username=testuser" \
  -d "password=password")

echo $TOKEN_RESPONSE | jq .

ACCESS_TOKEN=$(echo $TOKEN_RESPONSE | jq -r .access_token)
```

The `sub` field inside the decoded JWT is the user's UUID — this becomes `UserToken.id` in the backend.

To inspect the token payload:

```bash
echo $ACCESS_TOKEN | cut -d. -f2 | base64 -d 2>/dev/null | jq .
```

---

## 5. Call the API

The backend runs on port **6969** by default.

```bash
# Authenticated request
curl -s http://localhost:6969/some-endpoint \
  -H "Authorization: Bearer $ACCESS_TOKEN" | jq .

# Health check (no auth required)
curl -s http://localhost:6969/
```

---

## 6. Refresh and expiry

The token response also contains a `refresh_token`. Use it to get a new access token without re-entering credentials:

```bash
NEW_TOKEN=$(curl -s -X POST \
  http://localhost:8090/realms/master/protocol/openid-connect/token \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "grant_type=refresh_token" \
  -d "client_id=backend" \
  -d "refresh_token=$(echo $TOKEN_RESPONSE | jq -r .refresh_token)" \
  | jq -r .access_token)
```

---

## 7. Troubleshooting

| Symptom | Likely cause |
|---------|-------------|
| `401 Unauthorized` on API call | Token expired, or the backend JWKS cache is stale — restart the backend to force a `refresh()` |
| `invalid sub UUID` error | Keycloak `sub` is not a UUID — unlikely with standard Keycloak but check the token payload with step 4 |
| `No 'kid' in token header` | Token was issued without a signing key ID — make sure you are using the `backend` client, not `admin-cli` |
| `No matching key found in JWKS` | Backend fetched JWKS before Keycloak fully started; restart the backend |
| Keycloak not reachable | Check `docker compose ps` — `postgres_keycloak` must be healthy before Keycloak starts |
