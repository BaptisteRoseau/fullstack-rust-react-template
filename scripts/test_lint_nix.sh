#!/usr/bin/env bash
set -e

GIT_ROOT=$(git rev-parse --show-toplevel)
cd "$GIT_ROOT"

if ! command -v nixfmt >/dev/null; then
    echo "nixfmt is not installed, skipping the nix manifests" >&2
    echo "Run this script from 'nix develop' to get it" >&2
    exit 0
fi

NIX_FILES=$(find "$GIT_ROOT" -name "*.nix" -not -path "*/node_modules/*" -not -path "*/target/*")

nixfmt --check $NIX_FILES

statix check "$GIT_ROOT"

deadnix --fail "$GIT_ROOT"

nix flake check --no-build
