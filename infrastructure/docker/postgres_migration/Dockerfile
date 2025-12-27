FROM rust:1.92 AS builder

RUN RUSTFLAGS='-C target-feature=+crt-static' cargo install \
        --all-features --target-dir /temp sqlx-cli && \
    cp $(which sqlx-cli) /opt/sqlx-cli && \
    rm -r /temp && \
    chmod 555 /opt/sqlx-cli

FROM scratch

USER 1000
COPY --from=builder --chown=root:root /opt/sqlx-cli /sqlx-cli

ENTRYPOINT [ "/sqlx-cli" ]
CMD [ "--help" ]
