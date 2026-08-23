#!/usr/bin/env bash
set -e

GIT_ROOT=$(git rev-parse --show-toplevel)
cd "$GIT_ROOT"

# Verifies a page is complete and wired into the router.
# Usage: .claude/skills/frontend-page/scripts/check_page.sh <PageName>

PAGE=${1:?usage: check_page.sh <PageName>}
DIR="frontend/src/pages/$PAGE"
ROUTES=frontend/src/router/routes.tsx
CONSTANTS=frontend/src/router/constants.ts

failures=0

check() {
    if eval "$2" > /dev/null 2>&1; then
        echo "  ✓ $1"
    else
        echo "  ✗ $1"
        failures=$((failures + 1))
    fi
}

echo "Page folder:"
check "$DIR/$PAGE.tsx exists" "[ -f '$DIR/$PAGE.tsx' ]"
check "$DIR/$PAGE.test.tsx exists" "[ -f '$DIR/$PAGE.test.tsx' ]"
check "a *.module.scss exists" "ls '$DIR'/*.module.scss"
check "index.ts exports $PAGE" "grep -qE '\b$PAGE\b' '$DIR/index.ts'"

echo "Router:"
check "routes.tsx lazily imports @/pages/$PAGE" "grep -q \"@/pages/$PAGE'\" '$ROUTES'"
check "routes.tsx uses PATHS constants only" "! grep -qE \"path: *'\" '$ROUTES'"
check "constants.ts declares at least one path" "grep -q 'PATHS' '$CONSTANTS'"

echo "Navigation:"
# `to=` is in-app navigation and must come from PATHS. `href=` is left alone: it
# also targets backend routes such as /api/swagger.
check "no route literal in a to= prop" \
    "! grep -rEn \"to=['\\\"]/\" frontend/src --include='*.tsx'"

if [ "$failures" -ne 0 ]; then
    echo "$failures problem(s) found for page $PAGE."
    exit 1
fi

echo "Page $PAGE is wired up."
