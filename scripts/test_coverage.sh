#!/usr/bin/env bash
set -e

GIT_ROOT=$(git rev-parse --show-toplevel)
cd "$GIT_ROOT"

# Backend
cargo llvm-cov --color always --no-fail-fast --show-missing-lines \
    --ignore-filename-regex='backend/(models/|logging)'

# Frontend
cd frontend
bun test --coverage
