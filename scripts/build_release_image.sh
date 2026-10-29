#!/usr/bin/env bash
set -e

GIT_ROOT=$(git rev-parse --show-toplevel)
cd "$GIT_ROOT"

# Usage: build_release_image.sh <image> <dockerfile> [<prebuilt stage> <prebuilt directory>]
#
# Builds a release image from <dockerfile>, with the repository root as
# context, on the current buildx builder, and pushes $CI_REGISTRY_IMAGE/<image>:
# - from a tag, as <short sha> and <tag>. When the default branch already
#   pushed <short sha>, that image gets the tag instead of a new build, so the
#   release is the image that was tested.
# - from the default branch, as <short sha> and latest.
# - from any other branch, nowhere: the image is built, then dropped.
#
# Layers come from the registry cache $BUILD_CACHE:<image>, written only when
# CACHE_POLICY is pull-push. BuildKit has no image store, so a Dockerfile
# copying from localhost/tmp_health_checker gets that image built first and
# passed as a build context of that name. When <prebuilt directory> exists, it
# replaces the <prebuilt stage> of the Dockerfile, which is then not built.

USAGE="Usage: build_release_image.sh <image> <dockerfile> [<prebuilt stage> <prebuilt directory>]"
IMAGE_NAME=${1:?$USAGE}
DOCKERFILE=${2:?$USAGE}
PREBUILT_STAGE=${3:-}
PREBUILT_DIRECTORY=${4:-}
IMAGE="${CI_REGISTRY_IMAGE:?}/$IMAGE_NAME"
SHORT_SHA=${CI_COMMIT_SHORT_SHA:?}

cache_arguments() {
    local cache_reference="${BUILD_CACHE:?}:$1"
    CACHE_ARGUMENTS=(--cache-from "type=registry,ref=$cache_reference")
    if [ "${CACHE_POLICY:-pull}" = pull-push ]; then
        CACHE_ARGUMENTS+=(--cache-to "type=registry,ref=$cache_reference,mode=max")
    fi
}

if [ -n "${CI_COMMIT_TAG:-}" ] && docker buildx imagetools inspect "$IMAGE:$SHORT_SHA" >/dev/null 2>&1; then
    echo "$IMAGE:$SHORT_SHA is already pushed, tagging it $CI_COMMIT_TAG"
    docker buildx imagetools create --tag "$IMAGE:$CI_COMMIT_TAG" "$IMAGE:$SHORT_SHA"
    exit 0
fi

BUILD_CONTEXTS=()
if grep -q "localhost/tmp_health_checker" "$DOCKERFILE"; then
    HEALTH_CHECKER_DIRECTORY=$(mktemp -d)
    cache_arguments tmp_health_checker
    docker buildx build --output "type=local,dest=$HEALTH_CHECKER_DIRECTORY" "${CACHE_ARGUMENTS[@]}" \
        --file infrastructure/docker/tmp_health_checker/Dockerfile .
    BUILD_CONTEXTS+=(--build-context "localhost/tmp_health_checker:latest=$HEALTH_CHECKER_DIRECTORY")
fi
if [ -n "$PREBUILT_DIRECTORY" ] && [ -d "$PREBUILT_DIRECTORY" ]; then
    echo "Using $PREBUILT_DIRECTORY instead of building the $PREBUILT_STAGE stage"
    BUILD_CONTEXTS+=(--build-context "$PREBUILT_STAGE=$PREBUILT_DIRECTORY")
fi

OUTPUT_ARGUMENTS=(--push --tag "$IMAGE:$SHORT_SHA")
if [ -n "${CI_COMMIT_TAG:-}" ]; then
    OUTPUT_ARGUMENTS+=(--tag "$IMAGE:$CI_COMMIT_TAG")
elif [ "${CI_COMMIT_BRANCH:-}" = "${CI_DEFAULT_BRANCH:-main}" ]; then
    OUTPUT_ARGUMENTS+=(--tag "$IMAGE:latest")
else
    echo "Not on ${CI_DEFAULT_BRANCH:-main} nor on a tag, the image is not pushed"
    OUTPUT_ARGUMENTS=(--output type=cacheonly)
fi

cache_arguments "$IMAGE_NAME"
docker buildx build "${OUTPUT_ARGUMENTS[@]}" "${CACHE_ARGUMENTS[@]}" "${BUILD_CONTEXTS[@]}" \
    --file "$DOCKERFILE" .
