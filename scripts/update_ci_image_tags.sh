#!/usr/bin/env bash
set -e

GIT_ROOT=$(git rev-parse --show-toplevel)
cd "$GIT_ROOT"

# Usage: update_ci_image_tags.sh [--check]
#
# Tags each CI image of infrastructure/docker/ci_<name> with the hash of its
# files, in the <NAME>_CI_IMAGE variable of the GitLab entry point. Run it after
# changing a CI image directory, and commit the entry point with it. The
# ci-<name>-image jobs hash the directory the same way and fail on a stale tag.
#
# With --check, it changes nothing and fails when a tag is stale.

ENTRY_POINT=infrastructure/cicd/gitlab/gitlab-ci.yml
SHOULD_ONLY_CHECK=false
if [ "${1:-}" = "--check" ]; then
    SHOULD_ONLY_CHECK=true
fi

HAS_STALE_TAG=false
for directory in infrastructure/docker/ci_*; do
    name=${directory##*/ci_}
    variable="$(echo "$name" | tr '[:lower:]' '[:upper:]')_CI_IMAGE"
    hash=$(git ls-files -z -- "$directory" | LC_ALL=C sort -z | xargs -0 sha256sum | sha256sum | cut -c1-12)
    expected_line="  $variable: \$CI_REGISTRY_IMAGE/ci_$name:$hash"

    if grep -qxF -- "$expected_line" "$ENTRY_POINT"; then
        continue
    fi
    if ! grep -q "^  $variable: " "$ENTRY_POINT"; then
        echo "$ENTRY_POINT has no $variable variable for $directory" >&2
        exit 1
    fi
    if [ "$SHOULD_ONLY_CHECK" = true ]; then
        echo "$variable is stale: $directory hashes to $hash. Run ./scripts/update_ci_image_tags.sh" >&2
        HAS_STALE_TAG=true
        continue
    fi
    sed -i "s|^  $variable: .*|$expected_line|" "$ENTRY_POINT"
    echo "Tagged ci_$name with $hash"
done

if [ "$HAS_STALE_TAG" = true ]; then
    exit 1
fi
