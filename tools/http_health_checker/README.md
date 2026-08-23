# HTTP Health Checker

The lightest possible HTTP health checker for container health checks.

The checker takes a target URL as its last argument, requests it with a 3-second timeout,
and exits `0` when the response status code is 299 or below, `1` otherwise (including a
network failure or timeout). Built to be compiled statically and shipped inside any
container that needs a `HEALTHCHECK`.

## Keeping it thin

Size is the point of this crate, so [`main.rs`](src/main.rs) stays exactly as it is: one
match on one request, no helper functions, no error type, no logging, and `minreq` as its
only dependency. Do not factor anything out of it — there is nothing here big enough to be
worth a name, and every addition shows up in the image of every container that ships it.

Tests therefore live in [`tests/`](tests), not in `src/main.rs`. They start an axum server
on an ephemeral port and run the **compiled binary** against it (`CARGO_BIN_EXE_*`), so
what they check is the exit code a container's `HEALTHCHECK` actually sees, and they need
no internet. `axum` and `tokio` are dev-dependencies and never reach the release binary.
