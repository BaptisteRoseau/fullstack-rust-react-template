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

## Overrides

If you need local overrides, make them under [\<root>/docker-compose.override.yml](../../docker-compose.override.yml).

They will be applied to the docker images on your local environment.
