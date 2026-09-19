#!/usr/bin/env bash
set -e

GIT_ROOT=$(git rev-parse --show-toplevel)
cd "$GIT_ROOT"

KUBERNETES_VERSION=1.30.0

# Docker images
# --check parses and lints a Dockerfile without running the build, so the whole
# set is verified in seconds.
for dockerfile in infrastructure/docker/app_*/Dockerfile*; do
    echo "Checking $dockerfile"
    docker build --check --progress quiet --file "$dockerfile" .
done

# Docker compose
# config interpolates every variable and resolves every path without starting a
# container, so a missing variable or a bad mount path fails here. The profiles
# are all passed because a service behind a profile is otherwise never read.
# shellcheck source=/dev/null
source .env.dev
echo "Checking docker compose manifests"
docker compose \
    --profile debug \
    --profile debug-services \
    --profile release \
    config -q

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
