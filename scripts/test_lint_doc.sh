#!/usr/bin/env bash
set -e

GIT_ROOT=$(git rev-parse --show-toplevel)
cd "$GIT_ROOT"

MARKDOWN_FILES=$(find "$GIT_ROOT" -name "*.md")

markdownlint $MARKDOWN_FILES -c "$GIT_ROOT/.markdownlint.json"

# TODO: PR to support .gitignore and to ignore some URLs
markdown-link-check "$GIT_ROOT" $MARKDOWN_FILES --ignore frontend/node_modules

cspell lint "$GIT_ROOT" --gitignore -c "$GIT_ROOT/.cspell.json"
