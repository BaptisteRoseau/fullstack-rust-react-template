# Authentication — Keycloak

Keycloak is the identity provider. It hosts the login and registration pages, and issues the JWTs
the backend validates.

## Bootstrap

The realm is provisioned **automatically**. There is no manual admin-console setup.

| What | Where |
| --- | --- |
| Service definition | [docker-compose.authentication.yml](../../infrastructure/docker-compose/docker-compose.authentication.yml) |
| Realm definition | [realm-export.json](../../infrastructure/keycloak/import/realm-export.json) |

Keycloak runs with `start-dev --import-realm`, and the import directory is mounted read-only. On
first boot it imports every realm JSON it finds there.

The realm is named `app`. Keycloak listens on host port `8090`, and the admin console uses the
bootstrap credentials set in the compose file.

## The `app` realm

Read the current values in
[realm-export.json](../../infrastructure/keycloak/import/realm-export.json). What matters is why
each one is set:

| Setting | Why |
| --- | --- |
| `registrationAllowed` | Turns on the self-service registration page, so the app needs no sign-up form |
| `verifyEmail` | New users confirm their address. MailHog catches the mail, so no real SMTP server is needed |
| `accessTokenLifespan` | Deliberately short, so the silent refresh path is exercised in development |
| `sslRequired` | Set to allow plain HTTP on localhost. Change it for production |

### Clients

**`webapp`** is the confidential client the backend uses. It is *confidential* because the backend
is a server that can keep a secret and performs the code exchange itself.

- Authorization Code flow enabled, direct access grants disabled, PKCE `S256` required.
- Its redirect URI must match `AUTHENTICATOR_REDIRECT_URL`.
- An `oidc-audience-mapper` injects the API's audience into the access token. Without it, every
  token is rejected by the backend's audience validation.

**`backend`** is a public client with direct access grants enabled. The application flow does not
use it. It exists so integration tests and [manual-testing.md](./manual-testing.md) can fetch a raw
token with `curl`.

## What the backend reads

From the validated access token:

- `sub` — the user's UUID, surfaced as `UserToken.id`.
- `iss` — the issuer. Its last path segment becomes `UserToken.realm`.

`/auth/me` additionally calls Keycloak's `userinfo` endpoint for the display name and email.

## Changing the realm

`--import-realm` only imports a realm that does not already exist yet. To re-apply a changed realm
JSON in development, remove the Keycloak database volume and recreate the containers.

When refreshing the file from an admin-console export, check that **both** clients are still
present. A partial export silently drops the `clients` section, which leaves the realm without
`webapp` and `backend` and breaks the whole login flow.

For production, see the checklist in [configuration.md](./configuration.md).
