#!/usr/bin/env bash
set -e

GIT_ROOT=$(git rev-parse --show-toplevel)
cd "$GIT_ROOT"

# Builds the OpenAPI document straight from the backend router, no server needed.
# Pass an output path as the first argument (defaults to frontend/openapi.json).
cargo run --quiet -p openapi_generator -- "$@"
