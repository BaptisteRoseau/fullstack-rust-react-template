#!/usr/bin/env bash
set -e
GIT_ROOT=$(git rev-parse --show-toplevel)
VERSION=${VERSION:='1.0.0'}
TARGET=${TARGET:=x86_64-unknown-linux-gnu}
TARGET_DIR=${CARGO_TARGET_DIR:="$GIT_ROOT/target"}
PACKAGE_NAME="backend-$VERSION-$TARGET.tar.gz"

cd "$GIT_ROOT"
cargo build --release --target="$TARGET"

cd "$TARGET_DIR/$TARGET/release/"
mkdir -p backend
mv service crates/backend
tar czf "$PACKAGE_NAME" crates/
rm -r crates/
mv "$PACKAGE_NAME" "$OUTPUT_DIR/"
