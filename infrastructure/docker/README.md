# Docker

This directory contains Docker image file definitions.

## Image Definition Structure

```txt
.
└── app_<service>
    ├── Dockerfile
    └── <other assets embeded in the image>
```

The `<other assets embeded in the image>` contains every assets **embeded** in the final image.

For configs bound at runtime through a mounted volume or kubernetes config maps, use [infrastructure/configs](../configs).

For docker images where the image build differs between release and debug, use the following convention:

```txt
.
└── app_<service>
    ├── Dockerfile.debug
    ├── Dockerfile.release
    └── <other assets embeded in the images>
```

### Image Convention

- Use multi-stage builds to avoid intermediate objects in the final image
- Always use pinned images for `FROM` instructions, never `latest`
- Always specify a `USER` that is not root
- Always include exposed ports, commented with the exposed service (ex. `EXPOSE 8080 # web server`)
- Always include expected volumes (ex. `VOLUME [ "/var/lib/postgresql/data" ]`)
- Include a `HEALTHCHECK` when usefull. For HTTP services, use [tools/http_health_checker/Cargo.toml](../../tools/http_health_checker/Cargo.toml)
- For release images, always use the minimal image for the final image. If exposing a single binary, use the `FROM scratch` image.
- When compiling a binary, specify `ARG target=x86_64-unknown-linux-gnu`

For a complete example, read [./app_backend/Dockerfile.release]([./app_backend/Dockerfile.release]).

## Rules

- Every `docker/<dir>` is named `docker/app_<service>`
- Tag conventions:
    - Use short commit and latest version: `1.2.3-85e1cce`
    - Append `debug` to debug images: `1.2.3-85e1cce-debug`
    - Update `latest` image: `latest` and `latest-debug`
