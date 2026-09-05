# Docker Compose

This directory contains docker compose manifests mostly used for local development.

They are imported by the root docker-compose.yml file to splin up the application services.

## Profiles

(TODO: Follow this convention and find a way to enforce it)
Each manifest file is defined in `<profile>.docker-compose.yml`:

- [base.docker-compose.yml](./base.docker-compose.yml): The services required to run the application.
- [monitoring.docker-compose.yml](./monitoring.docker-compose.yml): Prometheus, Grafana, ...
- [debug.docker-compose.yml](./debug.docker-compose.yml): Hot reloading debug containers of the backend and frontend
- [release.docker-compose.yml](./release.docker-compose.yml): Release builds of the backend and frontend
- [debug-services.docker-compose.yml](./debug-services.docker-compose.yml): PgAdmin, Homepage, ...
