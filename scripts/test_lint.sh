#!/usr/bin/env bash
set -e

GIT_ROOT=$(git rev-parse --show-toplevel)
cd "$GIT_ROOT"

# TODO: CSpell & Typos & Markdownlint

# Backend
# --all-targets --all-features so tests and the feature-gated doubles are linted
# too, not just the default lib build.
# -A clippy::module_inception: several trait crates deliberately name a module
# after its parent directory (e.g. `database::database`) to keep the trait
# type's name matching the crate name; suppressed workspace-wide rather than
# per-file.
cargo clippy --workspace --all-targets --all-features -- -A clippy::module_inception

# Frontend
cd frontend
bunx eslint -c eslint.config.cjs --stats src
