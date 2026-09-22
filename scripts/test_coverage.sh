#!/usr/bin/env bash
set -e

GIT_ROOT=$(git rev-parse --show-toplevel)
cd "$GIT_ROOT"

# Backend
cargo llvm-cov --color always --no-fail-fast --show-missing-lines \
    --ignore-filename-regex='crates/(models/|logging)'

# Frontend
# `bun test` is bun's own runner: it would also pick up the Playwright specs
# under e2e/, which only `playwright test` can run. The project's runner is
# vitest, and vite.config.ts already scopes coverage to src/**.
cd frontend
bun run test --coverage
