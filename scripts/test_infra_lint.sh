#!/usr/bin/env bash
set -e

GIT_ROOT=$(git rev-parse --show-toplevel)
cd "$GIT_ROOT"

KUBERNETES_VERSION=1.30.0

# Docker images
# Every release image with an HTTP HEALTHCHECK copies the checker binary out of
# tmp_health_checker by image reference, so that image has to exist before the
# others can even be resolved. It is cached after the first build.
echo "Building infrastructure/docker/tmp_health_checker/Dockerfile"
docker build --progress quiet \
    --file infrastructure/docker/tmp_health_checker/Dockerfile \
    --tag localhost/tmp_health_checker:latest .

# --check parses and lints a Dockerfile without running the build, so the whole
# set is verified in seconds.
for dockerfile in infrastructure/docker/*/Dockerfile*; do
    echo "Checking $dockerfile"
    docker build --check --progress quiet --file "$dockerfile" .
done

# Docker compose
# config interpolates every variable and resolves every path without starting a
# container, so a missing variable or a bad mount path fails here. 'debug' and
# 'release' are two builds of the same services and share their container names,
# so they are checked one at a time rather than merged together.
# shellcheck source=/dev/null
source .env.dev
for profile in debug release; do
    echo "Checking docker compose manifests with the $profile profile"
    docker compose \
        --profile "$profile" \
        --profile debug-services \
        config -q
done

# Kubernetes
# LoadRestrictionsNone is required because the generated ConfigMaps read their
# content from infrastructure/configs rather than duplicating it.
if ! command -v kustomize >/dev/null; then
    echo "kustomize is not installed, skipping the kubernetes manifests" >&2
    exit 0
fi

for overlay in infrastructure/kubernetes/overlays/*/; do
    echo "Checking $overlay"
    rendered=$(kustomize build --load-restrictor LoadRestrictionsNone "$overlay")

    if command -v kubeconform >/dev/null; then
        echo "$rendered" |
            kubeconform -summary -strict -kubernetes-version "$KUBERNETES_VERSION"
    else
        echo "kubeconform is not installed, only checking that the overlay builds" >&2
    fi
done
