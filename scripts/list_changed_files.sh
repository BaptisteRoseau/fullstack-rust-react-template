#!/usr/bin/env bash
set -e

# Usage: list_changed_files.sh [--all-on-delete] [pathspec...]
#
# Prints the tracked files matching the pathspecs that changed since the branch
# left the default branch. Prints every matching file instead when
# SHOULD_RUN_ALL_JOBS is "true", or, with --all-on-delete, when a file was
# deleted or renamed: that can break a file which did not change, such as a
# markdown link.
#
# Unlike the other scripts, this one does not move to the git root: pathspecs and
# printed paths are relative to the current directory, so a job running in
# frontend/ gets paths it can hand straight to its linter.

ALL_ON_DELETE=false
if [ "${1:-}" = "--all-on-delete" ]; then
    ALL_ON_DELETE=true
    shift
fi

if [ "${SHOULD_RUN_ALL_JOBS:-false}" = true ]; then
    git ls-files -- "$@"
    exit 0
fi

DEFAULT_BRANCH=${CI_DEFAULT_BRANCH:-main}
if [ -n "${CI:-}" ]; then
    git fetch --quiet origin "+refs/heads/$DEFAULT_BRANCH:refs/remotes/origin/$DEFAULT_BRANCH"
fi
MERGE_BASE=$(git merge-base "origin/$DEFAULT_BRANCH" HEAD)

if [ "$ALL_ON_DELETE" = true ] &&
    [ -n "$(git diff --name-only --diff-filter=DR "$MERGE_BASE" HEAD)" ]; then
    git ls-files -- "$@"
    exit 0
fi

git diff --name-only --relative --diff-filter=d "$MERGE_BASE" HEAD -- "$@"
