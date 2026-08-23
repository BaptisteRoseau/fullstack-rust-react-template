#!/usr/bin/env bash
set -e

GIT_ROOT=$(git rev-parse --show-toplevel)
cd "$GIT_ROOT"

# Verifies unit tests sit in src/_tests/test_<name>.rs, declared with tests_file!.

failures=0

report() {
    echo "  ✗ $1"
    failures=$((failures + 1))
}

echo "Inline test modules:"
while read -r hit; do
    [ -z "$hit" ] && continue
    report "$hit declares an inline test module; move it to src/_tests/"
done < <(grep -rln '#\[cfg(test)\]' --include='*.rs' crates/*/src 2>/dev/null \
    | grep -v '/_tests/' \
    | xargs -r grep -ln 'mod tests\s*{\|mod test\s*{' 2>/dev/null || true)

echo "Hand-written #[path] triples:"
while read -r hit; do
    [ -z "$hit" ] && continue
    report "$hit hand-writes #[path = \"_tests/...\"]; use test_utils::tests_file! instead"
done < <(grep -rl 'path = "_tests/' --include='*.rs' crates/*/src 2>/dev/null || true)

echo "Test files reaching their parent module:"
while read -r f; do
    [ -z "$f" ] && continue
    # A wholly commented-out file is a test disabled on purpose; leave it alone.
    grep -qvE '^\s*(//.*)?$' "$f" || continue
    head -5 "$f" | grep -q 'use super::\*;' || report "$f does not start with 'use super::*;'"
done < <(find crates/*/src -path '*/_tests/test_*.rs' 2>/dev/null || true)

if [ "$failures" -ne 0 ]; then
    echo "$failures unit-test location problem(s) found."
    exit 1
fi

echo "Unit tests are all in the right place."
