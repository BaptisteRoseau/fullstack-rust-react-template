# Endpoints

The HTTP handlers. Each endpoint group is one directory holding the handlers and the types they
send and receive.

A handler extracts input, calls [app_core](../../../app_core), and serialises the result. Business
logic never lives here.

## Organization

```txt
<name>/
├── models.rs     # request and response types
├── endpoints.rs  # the handler functions
└── mod.rs        # only `pub mod endpoints;` and `pub mod models;`
```

Nested paths join their segments with `__`. CRUD endpoints for `/user/resources` live in
`user__resources/`.

## Models

Named `<Method><Resource><"Request" | "Response" | "Params">`. For `GET /user/{uuid}` the response
type is `GetUserResponse`; a path-only endpoint needs no params type.

Every model derives `Debug` and `#[serde(rename_all = "camelCase")]`, plus:

| Kind | Additional derives |
| --- | --- |
| Response | `Serialize`, `ToSchema`, `ToResponse` |
| Request body | `Deserialize`, `ToSchema` |
| Query or path parameters | `Deserialize`, `ToSchema`, `IntoParams` |

Conversions from `app_core` models are implemented here as `From<T>`, so handlers stay free of
mapping code.

## Handlers

Each handler carries a `#[utoipa::path(...)]` attribute declaring its method, path and every
response it can return. That attribute is the source of the OpenAPI document, which in turn
generates the frontend SDK — so an undocumented response is a response the frontend cannot see.

The doc comment above a handler is published as user-facing API documentation.

Handlers return `Result<Json<T>, ApiError>`, or `Result<(StatusCode, Json<T>), ApiError>` when the
status code varies.

## API contract

These types are the end-user API contract. **Removing**, **renaming** or **changing the type of** a
field or enum key is a breaking change, and the commit must carry a `BREAKING CHANGES:` footer
listing each one, prefixed with `API:`.

## Skills

- [backend-add-api-endpoint](../../../../.claude/skills/backend-add-api-endpoint/SKILL.md)
- [frontend-api-sdk](../../../../.claude/skills/frontend-api-sdk/SKILL.md)
- [commit-messages](../../../../.claude/skills/commit-messages/SKILL.md)
