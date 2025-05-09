#!/usr/bin/env bash
set -e
GIT_ROOT=$(git rev-parse --show-toplevel)
VERSION=${VERSION:='1.0.0'}
TARGET=${TARGET:=x86_64-unknown-linux-gnu}
OUTPUT_DIR=${OUTPUT_DIR:="$GIT_ROOT/packages"}
PACKAGE_NAME="frontend-$VERSION-$TARGET.tar.gz"

cd "$GIT_ROOT/frontend"
bunx vite build --outDir frontend

tar czf "$PACKAGE_NAME" frontend/
rm -r frontend/
mv "$PACKAGE_NAME" "$OUTPUT_DIR/"
