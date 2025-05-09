#!/usr/bin/env bash
set -e

GIT_ROOT=$(git rev-parse --show-toplevel)
cd "$GIT_ROOT"

# Backend
cargo deny check \
    --allow duplicate \
    --allow unlicensed \
    --allow license-not-encountered \
    --allow index-failure

# Frontend
cd frontend
ALLOWED_LICENSES='
    (MIT OR Apache-2.0);
    Apache-2.0 WITH LLVM-exception;
    Apache-2.0;
    BSD-2-Clause;
    BSD-3-Clause;
    BSL-1.0;
    CC-BY-4.0;
    CC0-1.0;
    ISC;
    MIT;
    MIT*;
    MPL-2.0;
    OpenSSL;
    Python-2.0;
    Unicode-3.0;
    Unicode-DFS-2016;
    UNLICENSED;
    Zlib;
'
echo 'Checking for bun/npm packages license compatibility...'
bun x license-checker --onlyAllow "$ALLOWED_LICENSES" >/dev/null
