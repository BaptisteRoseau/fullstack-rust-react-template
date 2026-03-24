#!/usr/bin/env bash
set -euo pipefail
GIT_ROOT=$(git rev-parse --show-toplevel)
cd "$GIT_ROOT"
cargo test --features integration -p storage -- --nocapture
