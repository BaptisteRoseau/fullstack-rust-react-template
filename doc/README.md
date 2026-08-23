# Architecture

How the whole system fits together, and what happens when a request travels through it.

Three kinds of documentation exist in this repository. Use the right one:

| Question | Where to look |
| --- | --- |
| How does the system fit together? | Here |
| What belongs in this directory, and what may it import? | That directory's `README.md` |
| How do I perform this task, step by step? | A skill under [`.claude/skills`](../.claude/skills) |

## The system at a glance

```txt
┌─────────────┐   HTTPS    ┌──────────────────┐         ┌──────────────┐
│   Browser   │ ─────────▶ │  Rust backend    │ ──────▶ │  Keycloak    │
│  React SPA  │ ◀───────── │  Axum, port 8080 │ ◀────── │  identity    │
└─────────────┘  cookies   └──────────────────┘         └──────────────┘
                                    │
                    ┌───────────────┼───────────────┐
                    ▼               ▼               ▼
              ┌──────────┐   ┌────────────┐   ┌──────────┐
              │ Postgres │   │   Redis    │   │ SeaweedFS│
              │   data   │   │   cache    │   │   blobs  │
              └──────────┘   └────────────┘   └──────────┘
```

The browser never talks to Keycloak's API or to a datastore. Everything goes through the backend.

## Backend layers

```txt
api → app_core → database
```

`api` handles HTTP and nothing else. `app_core` holds the business rules. `database` holds SQL. An
inner layer never imports an outer one, so each can be read and tested alone.

Everything external — the identity provider, the cache, blob storage, email — sits behind a trait in
its own crate. The layers depend on the trait, never on a concrete backend, and each backend is
behind a Cargo feature.

Details: [crates/README.md](../crates/README.md).

## Frontend layers

```txt
pages/ → components/ → design-system/ → hooks/ · utils/ · types/
                    ↘ api/hooks/ → api/domains/<domain>/ → api/generated/
```

Folders are technical roles, not features. Dependencies flow downwards only, and ESLint enforces the
`api/` boundary.

`api/generated/` is produced from the backend's OpenAPI document and is never edited by hand, so the
two sides of the wire cannot drift.

Details: [frontend/docs/architecture](../frontend/docs/architecture/README.md).

## Life of an API request

What happens, in order, and where the code is.

1. **The SPA calls a hook.** A component calls a hook from `api/hooks/`, which is an SWR binding
   around a fetcher in `api/domains/<domain>/`.
2. **The fetcher calls the generated SDK**, through
   [`api/client.ts`](../frontend/src/api/client.ts). The client attaches cookies and adds the `/api`
   prefix.
3. **Middlewares run, outside in.** Declared in
   [`routes/middlewares.rs`](../crates/api/src/routes/middlewares.rs), the first layer declared is
   the outermost: sensitive headers are masked, a request id is generated and echoed back, the
   tracing span opens, CORS is checked against the configured frontend origin, the path is
   normalised, the timeout starts, the body is decompressed, and finally the rate limiter runs.
4. **The router matches.** [`routes/router.rs`](../crates/api/src/routes/router.rs) nests the whole
   typed API under `/api` and merges the Swagger UI.
5. **Extractors resolve the caller.** [`extractors/user.rs`](../crates/api/src/extractors/user.rs)
   reads the credential from an `Authorization` header or the `access_token` cookie, and asks the
   `Authenticator` to turn it into a `UserToken`.
6. **The endpoint runs.** Handlers live in `crates/api/src/endpoints/<name>/`. A handler reads input,
   calls `app_core`, and returns a typed response. It contains no business logic. See
   [endpoints/README.md](../crates/api/src/endpoints/README.md).
7. **`app_core` applies the rules**, using the traits it was given through `AppState`:
   `Database`, `Cache`, `Storage`, `Authenticator`.
8. **The response travels back out** through the same middlewares in reverse, so it is compressed,
   logged and given its request id.
9. **Errors become HTTP.** [`error/response.rs`](../crates/api/src/error/response.rs) maps each error
   to a status and a stable error id. An expired token becomes `401`, which is the signal the
   frontend uses to refresh silently.

Every handler shares one `AppState` holding the traits and the read-only `Config`.

## Deep dives

| Topic | Contents |
| --- | --- |
| [Authentication](./authentication/README.md) | The Backend-for-Frontend OIDC flow, end to end |

## Where things are

| Area | Entry point |
| --- | --- |
| Backend crates | [crates/README.md](../crates/README.md) |
| HTTP endpoints | [crates/api/src/endpoints/README.md](../crates/api/src/endpoints/README.md) |
| Configuration | [crates/config/README.md](../crates/config/README.md) |
| Frontend | [frontend/README.md](../frontend/README.md) |
| Frontend architecture | [frontend/docs/architecture](../frontend/docs/architecture/README.md) |
| Build, test and lint | [scripts/README.md](../scripts/README.md) |
