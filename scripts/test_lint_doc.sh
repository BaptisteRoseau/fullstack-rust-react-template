#!/usr/bin/env bash
set -e

GIT_ROOT=$(git rev-parse --show-toplevel)
cd "$GIT_ROOT"

MARKDOWN_FILES=$(find "$GIT_ROOT" -name "*.md")

if [ ! command -v "markdownlint" ]; then
    echo "Markdownlint is not installed, install it with:"
    echo ""
    echo "    bun install -g markdownlint-cli"
else
    bun run markdownlint $MARKDOWN_FILES -c "$GIT_ROOT/.markdownlint.json"
fi

if [ ! command -v "markdownlink" ]; then
    echo "Markdownlink is not installed, install it with:"
    echo ""
    echo "    bun install -g markdown-link-check"
else
    # TODO: PR to support .gitignore and to ignore some URLs
    bun run markdown-link-check "$GIT_ROOT" $MARKDOWN_FILES --ignore frontend/node_modules
fi

if [ ! command -v "cspell-cli" ]; then
    echo "CSpell is not installed, install it with:"
    echo ""
    echo "    bun install  -g git+https://github.com/streetsidesoftware/cspell-cli"
else
    bun run cspell-cli lint "$GIT_ROOT" --gitignore -c "$GIT_ROOT/.cspell.json"
fi


# TODO: CSpell & Typos & Markdownlint
