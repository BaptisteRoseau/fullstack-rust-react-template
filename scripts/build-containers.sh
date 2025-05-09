#!/usr/bin/env bash
GIT_ROOT=$(git rev-parse --show-toplevel)
VERSION=${VERSION:='1.0.0'}
TARGET=${TARGET:=x86_64-unknown-linux-gnu}
ENGINE=${ENGINE='docker'}
IMAGE_NAME=${IMAGE_NAME:='localhost/backend'}

export VERSION
export TARGET
export ENGINE
export IMAGE_NAME

"$GIT_ROOT/frontend/build-containers.sh"
"$GIT_ROOT/backend/build-containers.sh"
