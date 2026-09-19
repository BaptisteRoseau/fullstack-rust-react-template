FROM rust:1.98.1 AS builder

ARG target=x86_64-unknown-linux-gnu

WORKDIR /build

RUN RUSTFLAGS='-C target-feature=+crt-static' \
    cargo install sqlx-cli \
    --version 0.9.0 \
    --no-default-features \
    --features postgres,rustls \
    --target $target \
    --root /opt && \
    chmod 555 /opt/bin/sqlx

FROM scratch

COPY --from=builder /opt/bin/sqlx /sqlx
COPY crates/database/migrations/ /migrations/

USER 10001
WORKDIR /

ENTRYPOINT [ "/sqlx", "migrate", "run", "--source", "/migrations" ]
