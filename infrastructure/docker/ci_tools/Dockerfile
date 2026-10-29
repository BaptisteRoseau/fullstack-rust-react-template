# Every linter that is not tied to the Rust or Bun toolchains.
FROM hadolint/hadolint:v2.15.1 AS hadolint
FROM koalaman/shellcheck:v0.11.0 AS shellcheck

FROM node:24.22.2-bookworm-slim

SHELL ["/bin/bash", "-o", "pipefail", "-c"]

ARG KUSTOMIZE_VERSION=v5.8.3
ARG KUBECONFORM_VERSION=v0.8.0

RUN apt-get update && \
    apt-get install -y --no-install-recommends git curl ca-certificates python3-venv && \
    rm -rf /var/lib/apt/lists/*

RUN npm install --global --no-fund --no-audit \
    markdownlint-cli@0.50.0 \
    markdown-link-check@3.15.0 \
    cspell@10.4.0 && \
    npm cache clean --force

ENV PATH="/opt/venv/bin:$PATH"
RUN python3 -m venv /opt/venv && \
    pip install --no-cache-dir \
    typos==1.51.1 \
    yamllint==1.38.0 \
    check-jsonschema==0.38.2

COPY --from=hadolint /bin/hadolint /usr/local/bin/hadolint
COPY --from=shellcheck /bin/shellcheck /usr/local/bin/shellcheck

RUN curl -sSfL "https://github.com/kubernetes-sigs/kustomize/releases/download/kustomize%2F$KUSTOMIZE_VERSION/kustomize_${KUSTOMIZE_VERSION}_linux_amd64.tar.gz" | \
    tar -xz -C /usr/local/bin && \
    curl -sSfL "https://github.com/yannh/kubeconform/releases/download/$KUBECONFORM_VERSION/kubeconform-linux-amd64.tar.gz" | \
    tar -xz -C /usr/local/bin kubeconform
