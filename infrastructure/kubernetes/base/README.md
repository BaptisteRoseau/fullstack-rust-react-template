# Base

Contains the minimum container services required to run the application:

- Backend
- Frontend
- Keycloak
- Keycloak Postgres
- Migrate
- Postgres
- Redis
- Seaweedfs

One directory per component, each with its own `kustomization.yaml`. See
[../README.md](../README.md) for the conventions these manifests follow.

`migrate` is a `Job`, not a `Deployment`: it runs the SQLx migrations once against
`postgres` and exits. Its `ConfigMap` is generated from `crates/database/migrations`,
so the migrations are never duplicated here.

`keycloak` imports the realm from `infrastructure/keycloak/import`, the same file
Docker Compose mounts.
