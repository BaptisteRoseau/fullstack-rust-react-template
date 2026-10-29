# Bun and the git the frontend jobs need to compare a branch with the default one.
FROM oven/bun:1.4.4

RUN apt-get update && \
    apt-get install -y --no-install-recommends git ca-certificates && \
    rm -rf /var/lib/apt/lists/*
