# Bun, the Node.js that Vitest needs, and the git the frontend jobs need to
# compare a branch with the default one.
FROM node:24.22.2-bookworm-slim AS node

FROM oven/bun:1.4.4

COPY --from=node /usr/local/bin/node /usr/local/bin/node

RUN apt-get update && \
    apt-get install -y --no-install-recommends git ca-certificates && \
    rm -rf /var/lib/apt/lists/*
