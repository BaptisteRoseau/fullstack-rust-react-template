#!/usr/bin/env bash
set -e

# Usage: list_changed_files.sh [--all-on-delete] [--exclude <glob>] [glob...]
#
# Prints the tracked files matching the globs that changed since the branch
# left the default branch. Prints every matching file instead when
# SHOULD_RUN_ALL_JOBS is "true", or, with --all-on-delete, when a file was
# deleted or renamed: that can break a file which did not change, such as a
# markdown link.
#
# The globs use the syntax of GitLab's rules:changes, relative to the git root,
# such as `**/*.md` or `frontend/**/*.{css,json}`. Git does not read braces, so
# a `{a,b}` group is expanded here. Files matching an --exclude glob are left
# out. With no glob, every file matches.
#
# Unlike the other scripts, this one does not move to the git root: printed
# paths are relative to the current directory, so a job running in frontend/
# gets paths it can hand straight to its linter.

ALL_ON_DELETE=false
EXCLUDED_GLOBS=()
while [ $# -gt 0 ]; do
    case "$1" in
    --all-on-delete)
        ALL_ON_DELETE=true
        shift
        ;;
    --exclude)
        if [ -n "$2" ]; then
            EXCLUDED_GLOBS+=("$2")
        fi
        shift 2
        ;;
    *)
        break
        ;;
    esac
done

print_brace_expansion() {
    local glob=$1
    if [[ ! $glob =~ ^([^{]*)\{([^{}]*)\}(.*)$ ]]; then
        printf '%s\n' "$glob"
        return
    fi
    local prefix=${BASH_REMATCH[1]}
    local suffix=${BASH_REMATCH[3]}
    local alternatives
    IFS=, read -ra alternatives <<<"${BASH_REMATCH[2]}"
    for alternative in "${alternatives[@]}"; do
        print_brace_expansion "$prefix$alternative$suffix"
    done
}

PATHSPECS=()
for glob in "$@"; do
    while IFS= read -r expanded_glob; do
        PATHSPECS+=(":(top,glob)$expanded_glob")
    done < <(print_brace_expansion "$glob")
done
if [ ${#PATHSPECS[@]} -eq 0 ]; then
    PATHSPECS+=(":(top,glob)**")
fi
for glob in "${EXCLUDED_GLOBS[@]}"; do
    while IFS= read -r expanded_glob; do
        PATHSPECS+=(":(top,glob,exclude)$expanded_glob")
    done < <(print_brace_expansion "$glob")
done

if [ "${SHOULD_RUN_ALL_JOBS:-false}" = true ]; then
    git ls-files -- "${PATHSPECS[@]}"
    exit 0
fi

DEFAULT_BRANCH=${CI_DEFAULT_BRANCH:-main}
if [ -n "${CI:-}" ]; then
    git fetch --quiet origin "+refs/heads/$DEFAULT_BRANCH:refs/remotes/origin/$DEFAULT_BRANCH"
fi
MERGE_BASE=$(git merge-base "origin/$DEFAULT_BRANCH" HEAD)

if [ "$ALL_ON_DELETE" = true ] &&
    [ -n "$(git diff --name-only --diff-filter=DR "$MERGE_BASE" HEAD)" ]; then
    git ls-files -- "${PATHSPECS[@]}"
    exit 0
fi

git diff --name-only --relative --diff-filter=d "$MERGE_BASE" HEAD -- "${PATHSPECS[@]}"
