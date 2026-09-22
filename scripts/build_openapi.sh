#!/usr/bin/env bash
set -e

GIT_ROOT=$(git rev-parse --show-toplevel)
cd "$GIT_ROOT"

# Builds the OpenAPI document straight from the backend router, no server needed.
#
# It always writes ./openapi.json and takes no output path: `Config::parse()` runs
# clap over the whole of `std::env::args()` and `CliConfig` has a positional
# argument, so a path lands on that instead. build_frontend_api_sdk.sh moves the
# document where it wants it.
cargo run --quiet -p openapi_generator
