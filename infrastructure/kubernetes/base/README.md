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
`postgres` and exits. It runs the
[app_migration](../../docker/app_migration/Dockerfile) image, which embeds the SQL files at build
time, so this directory neither copies the migrations nor reads them from outside
[infrastructure](../..).

`keycloak` imports the realm from
[infrastructure/configs/keycloak/import](../../configs/keycloak/import), the same file Docker
Compose mounts.
