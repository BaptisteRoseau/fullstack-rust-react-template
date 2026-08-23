# API

The HTTP layer. Turns [app_core](../app_core) and the service crates into an Axum server:
routing, request/response models, extractors, middlewares and the OpenAPI document.

See [crates/README.md](../README.md) for the layer rule: `api` may use `app_core` and
`database`, and both remain hidden behind `AppState` (see below) rather than imported
directly by handlers.

## Public surface

- `AppState` ([app_state.rs](src/app_state.rs)) — the `Clone` struct injected into every
  handler via Axum's `State` extractor. Holds `Arc<RwLock<dyn Trait>>` for each service
  (`Database`, `Storage`, `Cache`, `Authenticator`) plus `Arc<Config>`, so a handler never
  knows which backend is behind the trait.
- `routes::public_routes` — builds the full `Router`, mounted at `/api`, with Swagger UI
  and the production middleware stack applied.
- `routes::try_metrics_routes` — the separate Prometheus `/metrics` router.
- `routes::openapi` — builds the `OpenApi` document with no running service required, so
  it can be serialized offline to generate the frontend SDK.
- `error::ApiError` — the crate-wide error enum. See the error convention in
  [crates/README.md](../README.md).

## Directory

```txt
api/
├── src/
│   ├── app_state.rs   # AppState and its FromRef impls
│   ├── endpoints/     # route handlers, one directory per resource — see endpoints/README.md
│   ├── extractors/    # custom Axum extractors — see extractors/README.md
│   ├── middlewares/   # tower layers not specific to one route (e.g. rate limiting)
│   ├── models.rs      # request-scoped types shared across endpoints (e.g. UserToken)
│   ├── routes/        # router assembly, middleware stack, OpenAPI document
│   └── error/         # ApiError and its HTTP response conversion
└── Cargo.toml
```

See [endpoints/README.md](src/endpoints/README.md) and
[extractors/README.md](src/extractors/README.md) for those two directories.

## Routes

[routes/router.rs](src/routes/router.rs) builds one `OpenApiRouter` shared by the real
server and the OpenAPI generator, then nests it under `/api`. Swagger UI lives at the
path from `config.swagger`.

[routes/middlewares.rs](src/routes/middlewares.rs) assembles the production `tower`
stack in a fixed order (sensitive headers, request id, tracing span, CORS, timeout,
(de)compression, rate limiting). The order is load-bearing — see the comments in that
file before reordering a layer.

## Errors

`ApiError` ([error/error.rs](src/error/error.rs)) wraps every error the backend can
produce and converts `From` the lower layers' error types. `error/response.rs` maps each
variant to the `ApiErrorResponse` JSON body and HTTP status actually sent to the caller —
the two are kept separate so the trace logged server-side can stay more detailed than
what the client receives.

## Skills

- [backend-add-api-endpoint](../../.claude/skills/backend-add-api-endpoint/SKILL.md)
- [backend-feature-gating](../../.claude/skills/backend-feature-gating/SKILL.md)
