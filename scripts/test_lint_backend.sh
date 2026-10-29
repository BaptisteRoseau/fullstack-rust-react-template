#!/usr/bin/env bash
set -e

GIT_ROOT=$(git rev-parse --show-toplevel)
cd "$GIT_ROOT"
cargo clippy --workspace --all-targets --all-features --workspace -- -A clippy::module_inception
