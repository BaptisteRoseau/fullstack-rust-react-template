# Extractors

Axum extractors: types that pull data out of an incoming request for a handler, for
example `Option<UserToken>` in [endpoints/README.md](../endpoints/README.md)'s example
handler. One file per extracted type, named after it in snake_case. The `Rejection` type
is always [`ApiError`](../error/error.rs).

## Optional vs. required extractors

- **Optional** — extraction can legitimately yield nothing (e.g. an anonymous request has
  no `UserToken`). The type implements `OptionalFromRequestParts` (the real extraction
  logic) and `FromRequestParts` (a thin wrapper turning `None` into a rejection), giving
  handlers both `Option<UserToken>` and `UserToken`. See [`user.rs`](user.rs).
- **Required only** — extraction without a value makes no sense (e.g. `AppState`, always
  present). The type implements only `FromRequestParts`. See [`app_state.rs`](app_state.rs).

Axum does not derive `FromRequestParts` from `OptionalFromRequestParts` for a custom
type, so both impls have to be written by hand when both forms are needed.
