#!/usr/bin/env bash
set -e

GIT_ROOT=$(git rev-parse --show-toplevel)
cd "$GIT_ROOT"

# TODO: CSpell & Typos & Markdownlint

# Backend
cargo clippy

# Frontend
cd frontend
bunx eslint -c eslint.config.js --stats src
