# Authentication — Keycloak

Keycloak is the identity provider. It hosts the login and registration pages and issues the
JWTs the backend validates.

## Bootstrap

The realm is provisioned **automatically** — there is no manual admin-console setup.

- Service definition: [`infrastructure/docker-compose/docker-compose.authentication.yml`](../../infrastructure/docker-compose/docker-compose.authentication.yml)
- Realm definition: [`infrastructure/docker-compose/keycloak/import/realm-export.json`](../../infrastructure/docker-compose/keycloak/import/realm-export.json)

Keycloak runs with `start-dev --import-realm` and the import directory is bind-mounted
read-only:

```yaml
command: start-dev --import-realm
volumes:
  - ./keycloak/import:/opt/keycloak/data/import:ro
```

On first boot Keycloak imports every realm JSON it finds in that directory. Keycloak listens
on `localhost:8090` (mapped from the container's `8080`), and the admin console is reachable
with `admin` / `admin`.

## The `app` realm

| Setting | Value | Why |
|---------|-------|-----|
| `registrationAllowed` | `true` | Enables the self-service registration page. |
| `verifyEmail` | `true` | New users must confirm their address; the mail is caught by Mailhog (http://localhost:8025), so no real SMTP server is needed. |
| `accessTokenLifespan` | `300` (seconds) | Short-lived tokens, to exercise the silent refresh. |
| `sslRequired` | `external` | Dev only — allows plain HTTP on localhost. |

### Clients

**`webapp`** — the confidential client used by the Backend-for-Frontend.

| Setting | Value |
|---------|-------|
| Access type | confidential (`publicClient: false`, client-secret auth) |
| Standard (authorization code) flow | enabled |
| Direct access grants | disabled |
| PKCE | `S256` required |
| Redirect URIs | `http://localhost:8080/api/auth/callback` (+ `127.0.0.1`) |
| Post-logout redirect | `http://localhost:3000/*` |
| Secret (dev) | `webapp-secret` |

It is *confidential* because the backend is a server that can keep a secret and performs the
code exchange. An **audience mapper** (`oidc-audience-mapper`) injects the `backend` audience
into the access token, so the API's audience validation (`AUTHENTICATOR_AUDIENCES=backend`)
accepts tokens minted for `webapp`.

**`backend`** — a public client with direct access grants enabled. It is kept for parity with
the integration tests and for fetching a raw token via `curl`
(see [manual-testing.md](./manual-testing.md)). The application flow does not use it.

## Token contents

The backend reads two claims from the validated access token:

- `sub` — the user's UUID, surfaced as `UserToken.id`.
- `iss` — the issuer; its last path segment becomes `UserToken.realm`.

`/auth/me` additionally calls Keycloak's `userinfo` endpoint to read `email`, `given_name`
and `family_name` for display.

## Customizing

- **Production secret**: replace the `webapp` client `secret` and pass it to the backend via
  `OIDC_CLIENT_SECRET` (never commit a real secret).
- **Token lifetimes**: tune `accessTokenLifespan` and the SSO session timeouts in the realm
  JSON.
- **Email verification / password reset**: set `verifyEmail: true` and configure SMTP in the
  realm once you have a mail server.
- **Branding / extra fields**: customize the Keycloak login theme and registration form — the
  application does not need to change, since registration is entirely Keycloak-hosted.

> Re-importing: `--import-realm` only imports a realm that does not already exist. To re-apply
> a changed realm JSON in development, remove the Keycloak database volume
> (`postgres_keycloak`) and recreate the containers.
>
> Beware when refreshing this file from an admin-console export: a partial export silently
> drops the `clients` section, which leaves the realm without `webapp` and `backend` and
> breaks the whole login flow. Always check that both clients are still present after
> re-exporting.
