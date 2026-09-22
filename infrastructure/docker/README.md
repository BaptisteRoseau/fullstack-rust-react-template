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

A `tmp_<name>` directory holds a build-only image. It is never deployed and never published: it
exists so several release images can `COPY --from` one artifact instead of each compiling it.
[tmp_health_checker](./tmp_health_checker/Dockerfile) is the one such image, and every release
image with an HTTP `HEALTHCHECK` takes the binary from it:

```dockerfile
COPY --from=localhost/tmp_health_checker:latest /http_health_checker /bin/http_health_checker
```

Because that is an image reference rather than a stage, it must be built first or the build fails
resolving it. `./scripts/test_lint_infra.sh` builds it before reading the other Dockerfiles.

For docker images where the image build differs between release and debug, use the following convention:

```txt
.
└── app_<service>
    ├── Dockerfile.debug
    ├── Dockerfile.release
    └── <other assets embeded in the images>
```

Images used exclusively for CI are prefixed `ci_*`.

### Image Convention

- Use multi-stage builds to avoid intermediate objects in the final image
- Always use pinned images for `FROM` instructions, never `latest`
- Always specify a `USER` that is not root, for images built here and for upstream images that run
  as root by default. An upstream image whose own entrypoint drops privileges keeps its entrypoint:
  pin the UID in the deployment manifest instead of overriding `USER`, or the entrypoint loses the
  rights it needs to prepare its data directory.
- Always include exposed ports, commented with the exposed service when there is more than one.
  Docker only reads `#` at the start of a line, so the comment goes above the instruction:

  ```dockerfile
  # API server ; Prometheus metrics
  EXPOSE 9876 9100
  ```

- Always include expected volumes (ex. `VOLUME [ "/var/lib/postgresql/data" ]`)
- Release images that keep running must declare a `HEALTHCHECK`. For HTTP services, use
  [tools/http_health_checker](../../tools/http_health_checker/Cargo.toml). Two kinds of image skip
  it: debug images, which are rebuilt and watched by hand and would pay for the health checker
  build stage on every hot reload, and one-shot images that run to completion, which have nothing
  left to poll — their exit code is the health signal.
- For release images, always use the minimal image for the final image. If exposing a single binary, use the `FROM scratch` image.
- When compiling a binary, specify `ARG target=x86_64-unknown-linux-gnu`

For a complete example, read [app_backend/Dockerfile.release](./app_backend/Dockerfile.release).

## Verification

Every Dockerfile must pass the build checker, which parses and lints it without running a build:

```bash
./scripts/test_lint_infra.sh
```

Run it after editing any image definition. It also covers the Compose manifests and the Kubernetes
overlays, and the `pre-push` hook runs it with the other `test_*.sh` scripts.

## Rules

- Every `docker/<dir>` is named `docker/app_<service>`, or `docker/tmp_<name>` for a build-only
  image that nothing deploys.
- Image names are `app_*` for images built by this repository, remote image names stay unchanged.
- A `tmp_*` image is only ever tagged `latest`: nothing deploys it, so it has no version to carry,
  and the conventions below do not apply to it.
- Difference between `debug` and `release` images lies in the `-debug` suffix in the tag, not the image name.
- Tag conventions:
    - The version is the latest git tag, or `0.0.0` when the repository has none.
    - The build tag is that version and the short commit: `1.2.3-85e1cce`.
    - Append `debug` to debug images: `1.2.3-85e1cce-debug`.
    - Publishing a build also moves the rolling tags `1.2.3`, `1.2`, `1` and `latest`, and their
      `-debug` equivalents, onto it.
    - Never deploy a rolling tag: manifests reference the immutable `1.2.3-85e1cce` form.

    ```bash
    version="$(git describe --tags --abbrev=0 2>/dev/null || echo 0.0.0)"
    commit="$(git rev-parse --short HEAD)"
    # 1.2.3-85e1cce 1.2.3 1.2 1 latest
    ```
