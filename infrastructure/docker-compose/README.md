# Docker Compose

This directory contains docker compose manifests mostly used for local development.

They are imported by the root docker-compose.yml file to splin up the application services.

## Categories

Each manifest file is defined in `<category>.docker-compose.yml`:

- [base.docker-compose.yml](./base.docker-compose.yml): The services required to run the application.
- [monitoring.docker-compose.yml](./monitoring.docker-compose.yml): Prometheus, Grafana, ...
- [debug.docker-compose.yml](./debug.docker-compose.yml): Hot reloading debug containers of the backend and frontend
- [release.docker-compose.yml](./release.docker-compose.yml): Release builds of the backend and frontend
- [debug-services.docker-compose.yml](./debug-services.docker-compose.yml): PgAdmin, Homepage, ...

## Profiles

Here are profiles usable when using `docker compose --profile <profile> up`:

- `debug`: Runs the debug images of the `backend` and `frontend` services, with hot-reloading. Use if you prefer running them in docker rather than on the host
- `debug-services`: Runs services used to monitor and troubleshoot the application (`pgadmin`, `grafana`, `prometheus`, ...)
- `release`: Runs the release images of the `backend` and `frontend` services, built as they are shipped

## Environment

Every variable the manifests read is defined in [\<root>/.env.dev](../../.env.dev), which is the
exhaustive list and the one to copy from. Only the credentials below have no sensible default and
must be set before the stack is useful; everything else is a port number that `.env.dev` already
fills in.

| Variable | Used by |
| --- | --- |
| `POSTGRES_DATABASE`, `POSTGRES_USER`, `POSTGRES_PASSWORD` | `postgres`, `migrate` |
| `POSTGRES_READWRITE_USER`, `POSTGRES_READWRITE_PASSWORD` | `postgres`, the backend |
| `POSTGRES_READONLY_USER`, `POSTGRES_READONLY_PASSWORD` | `postgres` |
| `KEYCLOAK_DATABASE`, `KEYCLOAK_DATABASE_USER`, `KEYCLOAK_DATABASE_PASSWORD` | `postgres_keycloak`, `keycloak` |
| `KEYCLOAK_ADMIN_USER`, `KEYCLOAK_ADMIN_PASSWORD` | `keycloak` bootstrap admin |
| `PGADMIN_DEFAULT_EMAIL`, `PGADMIN_DEFAULT_PASSWORD` | `pgadmin`, `debug-services` profile only |
| `GRAFANA_ADMIN_USER`, `GRAFANA_ADMIN_PASSWORD` | `grafana`, `debug-services` profile only |

Load them before running compose:

```bash
source .env.dev
docker compose up -d
```

## Verification

The manifests must merge cleanly with every profile active:

```bash
./scripts/test_infra_lint.sh
```

It parses and interpolates without starting anything, so it catches a missing variable, a bad
mount path or a malformed key. Run it after editing any manifest.

## Overrides

If you need local overrides, make them under [\<root>/docker-compose.override.yml](../../docker-compose.override.yml).

They will be applied to the docker images on your local environment.
