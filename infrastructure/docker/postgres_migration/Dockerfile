FROM rust:1.92 AS builder

RUN apt update && \
    apt install perl libssl-dev && \
    rustup target install x86_64-unknown-linux-musl

RUN RUSTFLAGS='-C target-feature=+crt-static' cargo install sqlx-cli \
    --all-features \
    --target x86_64-unknown-linux-musl && \
    cp $(which sqlx) /opt/sqlx && \
    chmod 555 /opt/sqlx

FROM debian:bookworm-slim

USER 1000
COPY --from=builder --chown=root:root /opt/sqlx /bin/sqlx

ENTRYPOINT [ "/bin/sqlx" ]
CMD [ "--help" ]
