#!/usr/bin/env bash
set -e

GIT_ROOT=$(git rev-parse --show-toplevel)
cd "$GIT_ROOT"

# Usage: test_lint.sh [backend|frontend], both when omitted.
SECTION=${1:-all}

# --all-targets --all-features so tests and the feature-gated doubles are linted
# too, not just the default lib build.
# -A clippy::module_inception: several trait crates deliberately name a module
# after its parent directory (e.g. `database::database`) to keep the trait
# type's name matching the crate name; suppressed workspace-wide rather than
# per-file.
if [ "$SECTION" != frontend ]; then
    cargo clippy --workspace --all-targets --all-features -- -A clippy::module_inception
fi

if [ "$SECTION" != backend ]; then
    cd frontend
    bun run lint
fi
