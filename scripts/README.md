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

## Build

Every `build_*.sh` script is used to build either the docker containers, the backend, the frontend or any other thing that needs to be built.