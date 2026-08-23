#!/usr/bin/env bash
set -e

GIT_ROOT=$(git rev-parse --show-toplevel)
cd "$GIT_ROOT"

# Builds the workspace with every optional feature OFF.
#
# test_lint.sh and test_units.sh both pass --all-features, so they never compile
# this configuration. It is what a consumer depending only on a trait gets, and
# the only way to catch code left unreachable behind a new feature gate.

cargo check --workspace --no-default-features --all-targets
