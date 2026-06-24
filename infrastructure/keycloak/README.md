# Keycloak — `application` realm

## Overview

Keycloak runs on port **8090** (mapped from container port 8080) and is started with `--import-realm`, which auto-imports `realm-export.json` on first boot.

- Admin console: http://localhost:8090 (credentials: `admin` / `admin`)
- Issuer: `http://localhost:8090/realms/application`
- JWKS endpoint: `http://localhost:8090/realms/application/protocol/openid-connect/certs`
- OpenID configuration: `http://localhost:8090/realms/application/.well-known/openid-configuration`

## Realm layout

| Setting | Value |
|---|---|
| Realm name | `application` |
| Registration | disabled |
| Default role | `USER` (auto-assigned to all new accounts) |

## Roles

| Role | Description |
|---|---|
| `USER` | Standard application user — assigned by default to every new account |
| `ADMIN` | Elevated administrator privileges |

## `backend` confidential client

| Field | Value |
|---|---|
| Client ID | `backend` |
| Client authenticator | `client-secret` |
| **Dev secret** | `dev-backend-secret-change-me` |
| Public client | `false` |
| Standard flow (authorization code) | enabled |
| Direct access grants | enabled (useful for integration tests) |
| Valid redirect URIs | `http://localhost:8080/auth/callback`, `http://localhost:8080/*` |
| Post-logout redirect URIs | `http://localhost:3000/*`, `http://localhost:8080/*` |
| Web origins | `http://localhost:3000`, `http://localhost:8080`, `+` |

> **Production note**: Change `dev-backend-secret-change-me` to a strong randomly-generated secret. Never commit production secrets to this file — inject them via environment variables or a secrets manager.

### Audience mapper (required)

The backend validates that every access token's `aud` claim contains `backend`. This is enforced by the `backend-audience` protocol mapper of type `oidc-audience-mapper`:

```
included.client.audience = backend
access.token.claim        = true
id.token.claim            = false
```

Without this mapper the backend will reject all tokens with a 401.

## Seeded test users

| Username | Password | Email | Roles |
|---|---|---|---|
| `testuser` | `password` | `testuser@example.com` | `USER` |
| `adminuser` | `password` | `adminuser@example.com` | `USER`, `ADMIN` |

Passwords are non-temporary (no forced reset on first login). UUIDs are fixed in the export file so they remain stable across re-imports.

## How `--import-realm` works

Keycloak's `--import-realm` flag scans `/opt/keycloak/data/import/` for `*.json` files and imports any realm that does not yet exist in the database. It runs on every startup but skips realms that are already present — it does **not** overwrite existing configuration. To force a re-import after changes, either drop the `keycloak` Postgres database or delete the realm from the admin console before restarting.

## Re-exporting after console edits

After making changes in the Keycloak admin console you can export the realm back to a file:

```bash
# Exec into the running container
docker compose exec keycloak bash

# Inside the container
/opt/keycloak/bin/kc.sh export \
  --dir /opt/keycloak/data/import \
  --realm application \
  --users realm_file
```

Then copy the exported file out if needed:

```bash
docker compose cp keycloak:/opt/keycloak/data/import/application-realm.json \
  infrastructure/keycloak/realm-export.json
```

Review the diff before committing — exported files include many generated fields that can create noisy diffs.
