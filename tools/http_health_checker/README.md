# HTTP Health Checker

The purpose of this tool is to have the lightest HTTP Health Checker as possible for container health checking.

This health checker should:

- Take a target URL as argument
- Return 0 on success
- Return 1 on failure
- Be compiled
- Be statically linked

## How To Build

First you need to install Rust and Cargo. Then run the following commands. Notice that your target may be different:

```bash
TARGET=x86_64-unknown-linux-gnu
cargo build --release --target "${TARGET}"
strip --strip-all "target/${TARGET}/release/http_health_checker"
```

This results in a 1.5 MB executable:

```bash
$ du -h target/${TARGET}/release/http_health_checker
1,5M    target/x86_64-unknown-linux-gnu/release/http_health_checker
```

## How to use

To use the health checker via CLI, simply give it a target as argument.

It will return 0 on success:

```bash
$ ./http_health_checker "https://www.google.com/"
$ echo $?
0
```

And 1 on failure:

```bash
$ ./http_health_checker "https://random.nonsense.local/false/route/parameter"
$ echo $?
1
```

## Dockerfile

To enable the health checking in your container, add the following command in your Dockerfile:

```dockerfile
HEALTHCHECK \
    --start-period=5s \
    --interval=10s \
    --timeout=10s \
    --retries=3 \
    CMD [ "/path/to/health_checker", "http://localhost:12345/ping" ]
```

This obviously assumes that:

- The health checker exists in `/path/to/health_checker`
- Your API runs in the same container on port 12345
- The route used to know if your main API is alive is `/ping`

You may use different arguments `HEALTHCHECK` depending on your needs.

### Full Dockerfile Example

Suppose you have and application `application` and the following directories:

```text
├── Cargo.toml
├── Dockerfile
├── health_checker
│   └── main.rs
└── src
    └── main.rs
```

You can use the following Dockerfile to build you application and the health checker alongside it:

```dockerfile
ARG target
FROM rust:1.85 as BUILDER

COPY src/  /app/src/
COPY health_checker/ /app/health_checker/
COPY Cargo.toml /app/Cargo.toml
RUN cd /app && \
    export RUSTFLAGS='-C target-feature=+crt-static' && \
    cargo build \
        --release \
        --target $target && \
    mv target/$target/release/application /opt/application && \
    strip --strip-all /opt/application && \
    mv target/$target/release/health_checker /opt/health_checker && \
    strip --strip-all /opt/health_checker


FROM gcr.io/distroless/static-debian10:nonroot

COPY --from=BUILDER --chown=nonroot:nonroot /opt/application /home/app/application
COPY --from=BUILDER --chown=nonroot:nonroot /opt/health_checker /home/app/health_checker
USER nonroot
WORKDIR /home/app/bin
EXPOSE 12345
HEALTHCHECK \
    --start-period=5s \
    --interval=10s \
    --timeout=10s \
    --retries=3 \
    CMD [ "/home/app/health_checker", "http://localhost:12345/ping" ]
ENTRYPOINT [ "/home/app/application", "--port", "12345" ]
```
