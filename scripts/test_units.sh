#!/usr/bin/env bash
set -e

GIT_ROOT=$(git rev-parse --show-toplevel)
cd "$GIT_ROOT"

# Backend
cargo test --workspace --all-features

# Frontend
cd frontend && bun run test
