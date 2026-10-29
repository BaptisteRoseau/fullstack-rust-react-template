#!/usr/bin/env bash
set -euo pipefail

GIT_ROOT=$(git rev-parse --show-toplevel)
cd "$GIT_ROOT"

# Regenerates the frontend's API client in two halves: the OpenAPI document,
# straight from the backend router (cargo), and the TypeScript SDK it describes
# (bun). Run it after changing anything under crates/api.
#
# The document itself is a build artifact and is not committed -- only
# frontend/src/api/generated is. Pass --check to regenerate the document and
# verify that the committed SDK still matches it, writing nothing; that is what
# scripts/test_openapi.sh runs. Pass a path to put the document elsewhere.
# Pass --from <document> to skip the cargo half and use a document built
# earlier, as the CI does with the one its Rust tests job built.
#
# The generator cannot take an output path of its own: `Config::parse()` runs
# clap over the whole of `std::env::args()`, and `CliConfig` has a positional
# argument, so the path is swallowed by the config parser. It always writes
# ./openapi.json, which this script then moves into place.

CHECK=false
if [ "${1:-}" = "--check" ]; then
    CHECK=true
    shift
fi

function generate_sdk() {
    cd "$GIT_ROOT/frontend"
    if [ "$CHECK" = true ]; then
        bun run api:check "$1"
    else
        bun run api:sdk "$1"
    fi
}

if [ "${1:-}" = "--from" ]; then
    generate_sdk "$(realpath "$2")"
    exit 0
fi

OPENAPI_OUTPUT=$(realpath -m "${1:-frontend/openapi.json}")
GENERATED_DOCUMENT="$GIT_ROOT/openapi.json"

# The generator's own output path is the destination only when the caller asked
# for it; otherwise it is scratch, must not clobber an existing file, and is
# cleaned up however this script exits.
if [ "$GENERATED_DOCUMENT" != "$OPENAPI_OUTPUT" ]; then
    if [ -e "$GENERATED_DOCUMENT" ]; then
        echo "$GENERATED_DOCUMENT already exists and would be overwritten." >&2
        echo "Remove it and rerun this script." >&2
        exit 1
    fi
    trap 'rm -f "$GENERATED_DOCUMENT"' EXIT
fi

./scripts/build_openapi.sh

if [ "$GENERATED_DOCUMENT" != "$OPENAPI_OUTPUT" ]; then
    mv "$GENERATED_DOCUMENT" "$OPENAPI_OUTPUT"
fi
echo "Wrote the OpenAPI document to $OPENAPI_OUTPUT"

generate_sdk "$OPENAPI_OUTPUT"
