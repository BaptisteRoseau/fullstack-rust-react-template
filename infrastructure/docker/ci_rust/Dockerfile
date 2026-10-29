# Toolchain components, cargo subcommands, and the Docker CLI and buildx of
# every Rust job.
FROM rust:1.98.1-bookworm

SHELL ["/bin/bash", "-o", "pipefail", "-c"]

ARG BINSTALL_VERSION=v1.26.1
ARG DOCKER_CLI_VERSION=29.9.2
ARG BUILDX_VERSION=v0.38.1
ARG BUILDX_SHA256=e25d5cd675092bb5d1d03632fdc46cdf3fede7b514dc5f023a105e00d3ce5ad4

RUN rustup component add rustfmt clippy llvm-tools-preview

RUN curl -L --proto '=https' --tlsv1.2 -sSf \
    "https://raw.githubusercontent.com/cargo-bins/cargo-binstall/$BINSTALL_VERSION/install-from-binstall-release.sh" | bash && \
    cargo binstall --no-confirm --locked \
    cargo-llvm-cov@0.9.1 \
    cargo-deny@0.20.2 \
    cargo-audit@0.22.2 \
    cargo-machete@0.9.2 \
    sccache@0.19.0 \
    sqlx-cli@0.9.0 \
    sql-gen@0.2.3 && \
    rm -rf /usr/local/cargo/registry /usr/local/cargo/git

RUN curl -sSfL "https://download.docker.com/linux/static/stable/x86_64/docker-$DOCKER_CLI_VERSION.tgz" | \
    tar -xz --strip-components=1 -C /usr/local/bin docker/docker

RUN mkdir -p /usr/local/libexec/docker/cli-plugins && \
    curl -sSfL -o /usr/local/libexec/docker/cli-plugins/docker-buildx \
    "https://github.com/docker/buildx/releases/download/$BUILDX_VERSION/buildx-$BUILDX_VERSION.linux-amd64" && \
    echo "$BUILDX_SHA256  /usr/local/libexec/docker/cli-plugins/docker-buildx" | sha256sum --check && \
    chmod +x /usr/local/libexec/docker/cli-plugins/docker-buildx
