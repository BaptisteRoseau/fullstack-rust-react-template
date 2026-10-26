#!/usr/bin/env bash
set -e

GIT_ROOT=$(git rev-parse --show-toplevel)
cd "$GIT_ROOT"

# Backend
# `cargo deny check` with no argument also runs the advisories check, which is
# what test_cve.sh is for, so a CVE would fail both scripts. `bans` enforces the
# crates banned in deny.toml for their GPL or AGPL licence.
cargo deny check licenses bans \
    --allow duplicate \
    --allow unlicensed \
    --allow license-not-encountered \
    --allow index-failure

# Frontend
cd frontend
# NEW, everything below the Zlib line:
#   BlueOak-1.0.0, 0BSD, MIT-0, (MIT OR CC0-1.0), (MPL-2.0 OR Apache-2.0),
#   (Apache-2.0 AND BSD-3-Clause) and "Public Domain,MIT" are permissive.
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
'
echo 'Checking for bun packages license compatibility...'
bun x license-checker --onlyAllow "$ALLOWED_LICENSES" >/dev/null

# --onlyAllow accepts a licence as soon as one allowed name appears in it, so
# "GPL-3.0 AND MIT" would pass. LGPL is not matched.
echo 'Checking for GPL and AGPL licensed bun packages...'
GPL_PACKAGES=$(bun x license-checker --csv | cut -d, -f1,2 | grep -E '(^|[^L])A?GPL' || true)
if [ -n "$GPL_PACKAGES" ]; then
    echo "GPL or AGPL licensed packages found:" >&2
    echo "$GPL_PACKAGES" >&2
    exit 1
fi
