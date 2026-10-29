# The Playwright browsers and their system libraries, with Bun and oxaria on top.
FROM oven/bun:1.4.4 AS bun

FROM mcr.microsoft.com/playwright:v1.62.1-noble

SHELL ["/bin/bash", "-o", "pipefail", "-c"]

ARG OXARIA_VERSION=0.1.1

RUN apt-get update && \
    apt-get install -y --no-install-recommends git curl ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=bun /usr/local/bin/bun /usr/local/bin/bun
RUN ln -s /usr/local/bin/bun /usr/local/bin/bunx

RUN curl -sSfL "https://github.com/BaptisteRoseau/oxaria/releases/download/$OXARIA_VERSION/oxaria-linux-amd64.tar.gz" | \
    tar -xz -C /usr/local/bin
