#!/usr/bin/env bash
GIT_ROOT=$(git rev-parse --show-toplevel)
VERSION=${VERSION:='1.0.0'}
TARGET=${TARGET:=x86_64-unknown-linux-gnu}
ENGINE=${ENGINE='docker'}
IMAGE_NAME=${IMAGE_NAME:='localhost/backend'}
OUTPUT_DIR=${OUTPUT_DIR:="$GIT_ROOT/packages"}

export VERSION
export TARGET
export ENGINE
export IMAGE_NAME
export OUTPUT_DIR

set -e
mkdir -p "$OUTPUT_DIR"
"$GIT_ROOT/frontend/build.sh"
"$GIT_ROOT/crates/build.sh"
echo "Packages have been built into $OUTPUT_DIR"
