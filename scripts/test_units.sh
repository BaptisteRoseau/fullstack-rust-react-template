#!/usr/bin/env bash
set -e

GIT_ROOT=$(git rev-parse --show-toplevel)
cd "$GIT_ROOT"

# Backend
# --all-features: test targets guarded by `required-features` (the `test-utils`
# doubles running their own trait suite) are skipped without it.
cargo test --workspace --all-features

# Frontend
cd frontend
bun test
