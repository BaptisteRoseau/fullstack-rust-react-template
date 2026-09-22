# Build-only image: every release image that declares an HTTP HEALTHCHECK copies
# the binary out of it, so the checker is compiled once instead of once per image.
FROM rust:1.98.1 AS builder

ARG target=x86_64-unknown-linux-gnu
WORKDIR /build

COPY tools/http_health_checker/ ./

RUN RUSTFLAGS='-C target-feature=+crt-static' \
    cargo build \
    --release \
    --target $target && \
    mv target/$target/release/http_health_checker /opt/http_health_checker && \
    chmod 555 /opt/http_health_checker

FROM scratch

COPY --from=builder /opt/http_health_checker /http_health_checker

USER 10001
