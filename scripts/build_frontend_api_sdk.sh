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
#
# The generator cannot take an output path of its own: `Config::parse()` runs
# clap over the whole of `std::env::args()`, and `CliConfig` has a positional
# argument, so the path is swallowed by the config parser. It always writes
# ./openapi.json, which this script then moves into place.
#
# The cargo half runs with a scrubbed environment. Config values reach the
# document -- `servers`, and `openIdConnectUrl` under `securitySchemes` -- so a
# developer with those variables exported would otherwise generate an SDK that
# nobody else reproduces.

CHECK=false
if [ "${1:-}" = "--check" ]; then
    CHECK=true
    shift
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

scrubbed_env=(env -i PATH="$PATH" HOME="$HOME")
if [ -n "${CARGO_TARGET_DIR:-}" ]; then
    scrubbed_env+=(CARGO_TARGET_DIR="$CARGO_TARGET_DIR")
fi
"${scrubbed_env[@]}" ./scripts/build_openapi.sh

if [ "$GENERATED_DOCUMENT" != "$OPENAPI_OUTPUT" ]; then
    mv "$GENERATED_DOCUMENT" "$OPENAPI_OUTPUT"
fi
echo "Wrote the OpenAPI document to $OPENAPI_OUTPUT"

cd frontend
if [ "$CHECK" = true ]; then
    bun run api:check "$OPENAPI_OUTPUT"
else
    bun run api:sdk "$OPENAPI_OUTPUT"
fi
