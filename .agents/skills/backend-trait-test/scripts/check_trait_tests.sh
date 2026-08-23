#!/usr/bin/env bash
set -e

GIT_ROOT=$(git rev-parse --show-toplevel)
cd "$GIT_ROOT"

# Verifies a trait crate's backends are each covered by a test binary.
# Usage: .claude/skills/backend-trait-test/scripts/check_trait_tests.sh <crate>

CRATE=${1:?usage: check_trait_tests.sh <crate>}
DIR="crates/$CRATE"
TOML="$DIR/Cargo.toml"

[ -d "$DIR/src/backends" ] || { echo "$CRATE has no src/backends — this skill does not apply."; exit 0; }

failures=0
report() { echo "  ✗ $1"; failures=$((failures + 1)); }

echo "Cargo.toml:"
if grep -q '^autotests *= *false' "$TOML"; then
    echo "  ✓ autotests = false"
else
    report "autotests = false is missing, so cargo will auto-discover test files"
fi

echo "Backend coverage:"
# A backend is either <name>.rs or a <name>/ directory.
for src in "$DIR"/src/backends/*; do
    b=$(basename "$src"); b=${b%.rs}
    [ "$b" = "mod" ] && continue
    [ "$b" = "_tests" ] && continue
    if [ ! -f "$DIR/tests/backends/$b.rs" ]; then
        report "src/backends/$b.rs has no tests/backends/$b.rs"
        continue
    fi
    grep -q "tests/backends/$b.rs" "$TOML" \
        || report "tests/backends/$b.rs has no [[test]] stanza in Cargo.toml"
    echo "  ✓ $b"
done

echo "Test binary shape:"
for t in "$DIR"/tests/backends/*.rs; do
    [ -e "$t" ] || continue
    grep -q 'test_trait_main!' "$t" || report "$t has no test_trait_main! invocation"
    grep -qE '^\s*pub ' "$t" && report "$t declares a pub item; keep every item private"
    grep -q 'harness *= *false' "$TOML" || true
done

if [ "$failures" -ne 0 ]; then
    echo "$failures trait-test problem(s) found in $CRATE."
    exit 1
fi

echo "$CRATE trait tests are complete."
