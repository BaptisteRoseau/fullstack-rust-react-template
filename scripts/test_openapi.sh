#!/usr/bin/env bash
set -e

GIT_ROOT=$(git rev-parse --show-toplevel)
cd "$GIT_ROOT"

# Fails when the committed frontend/src/api/generated no longer matches an
# OpenAPI document regenerated from the backend router. Needs cargo, which is
# why it is a script of its own rather than part of test_lint.sh.
./scripts/build_frontend_api_sdk.sh --check
