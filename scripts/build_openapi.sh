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
#
# It runs with a scrubbed environment. Config values reach the document --
# `servers`, and `openIdConnectUrl` under `securitySchemes` -- so a developer
# with those variables exported would otherwise generate an SDK that nobody else
# reproduces. What the scrub keeps is the toolchain's configuration: inside
# `nix develop` cargo finds openssl through it, and in the CI it carries the
# compiler wrappers and flags that let this build reuse the tests' build.
scrubbed_env=(env -i PATH="$PATH" HOME="$HOME")
while IFS= read -r variable; do
    case "$variable" in
    CARGO_* | RUSTUP_* | RUSTC_* | RUSTFLAGS | RUSTDOCFLAGS | SCCACHE_* | __CARGO_LLVM_COV* | LLVM_PROFILE_FILE | PKG_CONFIG_PATH | LD_LIBRARY_PATH)
        scrubbed_env+=("$variable=${!variable}")
        ;;
    esac
done < <(compgen -e)

"${scrubbed_env[@]}" cargo run --quiet -p openapi_generator
