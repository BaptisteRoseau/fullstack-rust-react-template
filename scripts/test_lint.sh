#!/usr/bin/env bash
set -e

GIT_ROOT=$(git rev-parse --show-toplevel)
cd "$GIT_ROOT"

# TODO: CSpell & Typos & Markdownlint

# Backend
# --all-targets --all-features so tests and the feature-gated doubles are linted
# too, not just the default lib build.
cargo clippy --workspace --all-targets --all-features

# Frontend
cd frontend
bunx eslint -c eslint.config.cjs --stats src
