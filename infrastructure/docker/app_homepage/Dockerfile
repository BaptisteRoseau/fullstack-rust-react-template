FROM rust:1.98.1 AS health_checker

ARG target=x86_64-unknown-linux-gnu
WORKDIR /build

COPY tools/http_health_checker/ ./http_health_checker/

RUN cd ./http_health_checker/ && \
    RUSTFLAGS='-C target-feature=+crt-static' \
    cargo build \
    --release \
    --target $target && \
    mv target/$target/release/http_health_checker /opt/http_health_checker && \
    chmod 555 /opt/http_health_checker

FROM ghcr.io/gethomepage/homepage:v1.4.0

COPY --from=health_checker /opt/http_health_checker /bin/http_health_checker

RUN mkdir -p /app/config/logs && \
    chown -R 1000:1000 /app/config

VOLUME [ "/app/config" ]

USER 1000:1000
EXPOSE 3000
HEALTHCHECK \
    --start-period=10s \
    --interval=10s \
    --timeout=10s \
    --retries=3 \
    CMD [ "/bin/http_health_checker", "http://127.0.0.1:3000/" ]
