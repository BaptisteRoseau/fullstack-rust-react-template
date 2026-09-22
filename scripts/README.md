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

### Dependency advisories

[`test_cve.sh`](./test_cve.sh) reports, it does not gate: it prints every advisory `cargo audit`
and `bun-audit` find and still exits 0. Advisories land in transitive dependencies, so the fix is
usually an upstream release rather than a commit here, and one published overnight should not stop
a push. Read its output and upgrade whatever has a fixed release.

### Infrastructure

[`test_lint_infra.sh`](./test_lint_infra.sh) checks every Dockerfile with `docker build --check`,
merges the Compose manifests with all profiles through `docker compose config`, and builds both
Kustomize overlays through `kubeconform`. None of it starts a container.

The Kubernetes part needs `kustomize` and `kubeconform`, which are not needed anywhere else in the
project. The script skips that part when they are missing rather than failing the `pre-push` hook,
so install them to get the full check:

```sh
go install sigs.k8s.io/kustomize/kustomize/v5@latest
go install github.com/yannh/kubeconform/cmd/kubeconform@latest
```

### Nix

[`test_lint_nix.sh`](./test_lint_nix.sh) checks every `.nix` file: `nixfmt` for formatting, `statix`
for anti-patterns, `deadnix` for unused bindings and arguments, then `nix flake check --no-build`
to evaluate every flake output without building it.

It evaluates the outputs one by one rather than running `nix flake check`, which takes every node
down to its `system.build.toplevel` and trips the assertion `base.nix` makes on
`infrastructure/nix/prod/authorized_keys`. That file is provisioned per machine and absent from a
fresh clone, so the nodes are evaluated with a lint-only key: everything except the key itself is
checked here, and the real one is what `nix build .#nixosConfigurations.<node>...` verifies at
deploy time. See [../infrastructure/nix/prod](../infrastructure/nix/prod).

All four come from the development shell, so the script skips itself when `nixfmt` is missing rather
than failing the `pre-push` hook of a developer who does not use Nix:

```sh
nix develop -c ./scripts/test_lint_nix.sh
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
