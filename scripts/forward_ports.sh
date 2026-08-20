#!/usr/bin/env bash
set -e

GIT_ROOT=$(git rev-parse --show-toplevel)
cd "$GIT_ROOT"

# Forwards every port published by the docker-compose manifests to the host,
# via `sbx ports`. Run this from your host machine (not inside the sandbox):
# `sbx` drives the sandbox from the outside.

function ensure_installed() {
    executable=$1
    if [ ! -x "$(command -v "$executable")" ]; then
        echo "Missing $executable, please install it and rerun this script" >&2
        exit 1
    fi
}

ensure_installed "yq"
ensure_installed "sbx"

# shellcheck disable=SC1091
source .env

SANDBOX_NAME="claude-$(basename "$GIT_ROOT")"

# Every docker-compose manifest included by the root compose file.
mapfile -t COMPOSE_FILES < <(yq -r '.include[]' docker-compose.yml)

# Collect every published (host-side) port across all services and manifests,
# expanding "${VAR}" references against the sourced .env, then deduplicate.
mapfile -t PORTS < <(
    for file in "${COMPOSE_FILES[@]}"; do
        yq -r '.services[].ports[]?' "$file"
    done \
        | while IFS= read -r mapping; do
            [ -n "$mapping" ] && eval echo "$mapping"
        done \
        | cut -d: -f1 \
        | sort -un
)

if [ ${#PORTS[@]} -eq 0 ]; then
    echo "No ports found in the docker-compose manifests." >&2
    exit 1
fi

PUBLISH_ARGS=()
for port in "${PORTS[@]}"; do
    PUBLISH_ARGS+=(--publish "${port}:${port}/tcp")
done

echo "Forwarding ${#PORTS[@]} port(s) to sandbox $SANDBOX_NAME: ${PORTS[*]}"
set -x
sbx ports "$SANDBOX_NAME" "${PUBLISH_ARGS[@]}"
