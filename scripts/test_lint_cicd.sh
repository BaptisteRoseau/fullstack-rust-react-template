#!/usr/bin/env bash
set -e

GIT_ROOT=$(git rev-parse --show-toplevel)
cd "$GIT_ROOT"

# Usage: test_lint_cicd.sh [file.gitlab-ci.yml...], every GitLab CI manifest when omitted.
#
# Validates each file alone against the GitLab CI schema: it does not resolve
# `include:`, so a job extending a template from another file is not checked
# against it. `glab ci lint` does that, but needs a GitLab instance and a token.
# It also fails when a CI image tag no longer matches its directory.
./scripts/update_ci_image_tags.sh --check

if ! command -v check-jsonschema >/dev/null; then
    echo "check-jsonschema is not installed, skipping the CI/CD manifests" >&2
    echo "Install it with 'pipx install check-jsonschema'" >&2
    exit 0
fi

if [ $# -gt 0 ]; then
    CICD_FILES=("$@")
else
    mapfile -t CICD_FILES < <(git ls-files ".gitlab-ci.yml" "infrastructure/cicd/gitlab/gitlab-ci.yml" "infrastructure/cicd/gitlab/*.gitlab-ci.yml")
fi

# A file with a `spec:` header holds two YAML documents, which check-jsonschema
# cannot read, so each document is checked from its own copy.
SPLIT_DIR=$(mktemp -d)
trap 'rm -rf "$SPLIT_DIR"' EXIT
FILES_TO_CHECK=()
for file in "${CICD_FILES[@]}"; do
    if ! grep -qx -- "---" "$file"; then
        FILES_TO_CHECK+=("$file")
        continue
    fi
    copy="$SPLIT_DIR/${file//\//__}"
    awk -v header="$copy.header.yml" -v body="$copy.body.yml" \
        '$0 == "---" { in_body = 1; next } { print > (in_body ? body : header) }' "$file"
    echo "Checking $file as $copy.header.yml and $copy.body.yml"
    FILES_TO_CHECK+=("$copy.header.yml" "$copy.body.yml")
done

check-jsonschema --builtin-schema vendor.gitlab-ci --data-transform gitlab-ci "${FILES_TO_CHECK[@]}"
