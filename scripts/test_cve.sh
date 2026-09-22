#!/usr/bin/env bash
set -e

GIT_ROOT=$(git rev-parse --show-toplevel)
cd "$GIT_ROOT"

# This script reports, it does not gate. An advisory almost always lands in a
# transitive dependency, so the fix is an upstream release rather than a commit
# here, and an advisory published overnight should not stop a push. Read the
# output, upgrade what has a fix, and leave the rest until upstream moves.
FOUND=false

# Backend
cargo audit || FOUND=true

# Frontend
(cd frontend && bun x bun-audit) || FOUND=true

if [ "$FOUND" = true ]; then
    echo
    echo "Vulnerable dependencies found, see the advisories above." >&2
    echo "Upgrade the crates and packages that have a fixed release." >&2
    echo "Not failing: the remaining ones wait on an upstream release." >&2
fi
