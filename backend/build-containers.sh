#!/usr/bin/env bash
SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
GIT_ROOT=$(git rev-parse --show-toplevel)
VERSION=${VERSION:='1.0.0'}
TARGET=${TARGET:=x86_64-unknown-linux-gnu}
ENGINE=${ENGINE='docker'}
IMAGE_NAME=${IMAGE_NAME:='localhost/backend'}

"$ENGINE" build \
    --file "$SCRIPT_DIR/Dockerfile.release" \
    --build-arg target="$TARGET" \
    -t "$IMAGE_NAME:$VERSION" \
    -t "$IMAGE_NAME:latest" \
    "$GIT_ROOT"

"$ENGINE" build \
    --file "$SCRIPT_DIR/Dockerfile.debug" \
    --build-arg target="$TARGET" \
    -t "$IMAGE_NAME:$VERSION-debug" \
    -t "$IMAGE_NAME:latest-debug" \
    "$GIT_ROOT"
