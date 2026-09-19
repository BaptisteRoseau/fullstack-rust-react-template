# Scripts

This directory contains helpers and scripts to test, build and lint the platform.

## Conventions

Every script should start with the following lines:

```sh
#!/usr/bin/env bash
set -e

GIT_ROOT=$(git rev-parse --show-toplevel)
cd "$GIT_ROOT"
```

This is to make sure the scripts can be run from anywhere and will exit on error.

## Git Hooks

The git hooks are located in [git_hooks](./git_hooks). Run the [git_hooks/setup.sh](./git_hooks/setup.sh) to enable them in the project. We use a symlink to allow to edit the hooks without having to re-run the script on modification.

## Tests

Every `test_*.sh` script is used in the `pre-push` hook or can be used as a standalone to test the platform. For example:

- Unit tests
- Coverage
- Dependencies CVEs
- Licences compliance
- Linter results
- Infrastructure manifests

### Infrastructure

[`test_infra_lint.sh`](./test_infra_lint.sh) checks every Dockerfile with `docker build --check`,
merges the Compose manifests with all profiles through `docker compose config`, and builds both
Kustomize overlays through `kubeconform`. None of it starts a container.

The Kubernetes part needs `kustomize` and `kubeconform`, which are not needed anywhere else in the
project. The script skips that part when they are missing rather than failing the `pre-push` hook,
so install them to get the full check:

```sh
go install sigs.k8s.io/kustomize/kustomize/v5@latest
go install github.com/yannh/kubeconform/cmd/kubeconform@latest
```

## Build

Every `build_*.sh` script is used to build either the docker containers, the backend, the frontend or any other thing that needs to be built.

### The frontend API SDK

[`build_frontend_api_sdk.sh`](./build_frontend_api_sdk.sh) regenerates `frontend/src/api/generated/`
from the OpenAPI document the `api` crate emits. Run it after any change under `crates/api`, and
commit the generated folder — `frontend/openapi.json` is a build artifact and is gitignored.

[`test_openapi.sh`](./test_openapi.sh) is the matching gate: it fails when the committed SDK no
longer matches the router. Both need cargo *and* bun, which is why the check is not part of
`test_lint.sh`'s frontend section.