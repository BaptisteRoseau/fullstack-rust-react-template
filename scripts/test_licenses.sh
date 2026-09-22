#!/usr/bin/env bash
set -e

GIT_ROOT=$(git rev-parse --show-toplevel)
cd "$GIT_ROOT"

# Backend
# `cargo deny check` with no argument also runs the advisories check, which is
# what test_cve.sh is for, so a CVE would fail both scripts.
cargo deny check licenses \
    --allow duplicate \
    --allow unlicensed \
    --allow license-not-encountered \
    --allow index-failure

# Frontend
cd frontend
# NEW, everything below the Zlib line:
#   BlueOak-1.0.0, 0BSD, MIT-0, (MIT OR CC0-1.0), (MPL-2.0 OR Apache-2.0),
#   (Apache-2.0 AND BSD-3-Clause) and "Public Domain,MIT" are permissive.
#   Apache* is how license-checker reports the @pm2/* packages' Apache licence.
#   AGPL-3.0 is pm2 itself, a transitive dev dependency that no browser bundle
#   ships.
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
    BlueOak-1.0.0;
    0BSD;
    MIT-0;
    (MIT OR CC0-1.0);
    (MPL-2.0 OR Apache-2.0);
    (Apache-2.0 AND BSD-3-Clause);
    Public Domain,MIT;
    Apache*;
    AGPL-3.0;
'
echo 'Checking for bun packages license compatibility...'
bun x license-checker --onlyAllow "$ALLOWED_LICENSES" >/dev/null
