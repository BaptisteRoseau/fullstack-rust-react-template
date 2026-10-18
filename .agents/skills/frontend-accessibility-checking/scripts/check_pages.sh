#!/usr/bin/env bash
set -e

# Runs oxaria on each URL and lists the ones that still have findings.
# Usage: .claude/skills/frontend-accessibility-checking/scripts/check_pages.sh [-H <header>]... <url>...
# Every -H is passed to each oxaria call, e.g. -H "Cookie: access_token=...; refresh_token=...".

usage="usage: check_pages.sh [-H <header>]... <url>..."
headers=()
urls=()

while [ $# -gt 0 ]; do
    case "$1" in
        -H | --header)
            headers+=(-H "${2:?$usage}")
            shift 2
            ;;
        *)
            urls+=("$1")
            shift
            ;;
    esac
done

[ ${#urls[@]} -gt 0 ] || { echo "$usage" >&2; exit 2; }

failing=()
for url in "${urls[@]}"; do
    if output=$(oxaria "${headers[@]}" "$url" 2>&1); then
        echo "  ✓ $url"
    else
        echo "  ✗ $url"
        echo "$output" | sed 's/^/      /'
        failing+=("$url")
    fi
done

echo
if [ ${#failing[@]} -gt 0 ]; then
    echo "${#failing[@]}/${#urls[@]} page(s) still fail: ${failing[*]}"
    exit 1
fi
echo "All ${#urls[@]} page(s) pass."
