---
name: backend-add-api-endpoint
description: Use when adding, changing or removing an HTTP endpoint in the Rust API (crates/api).
---

# Add a backend API endpoint

The `api` crate handles HTTP only: extract input, call `app_core`, serialise the result. **Business
logic goes in [app_core](../../../crates/app_core), never in a handler.** If you are writing domain
rules inside a handler, move them first.

Conventions for these files are described in
[endpoints/README.md](../../../crates/api/src/endpoints/README.md).

## 1. Create the directory

One directory per endpoint group, under `crates/api/src/endpoints/<name>/`. For a nested path such
as `/user/resources`, join the segments with `__`: `user__resources`.

Copy the three templates in [assets/](./assets) into it, then rename the types:

```bash
cp .claude/skills/backend-add-api-endpoint/assets/{mod.rs,models.rs,endpoints.rs} \
   crates/api/src/endpoints/<name>/
```

## 2. Fill in `models.rs`

One struct per request body, response body or parameter set, named
`<Method><Resource><"Request"|"Response"|"Params">`.

The derives and the `camelCase` rule are in the template. Every doc comment is published in the
OpenAPI document, so write it for the API consumer, not for yourself.

Put `From<DomainType>` conversions here, so the handler stays free of mapping code.

```rust
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToResponse, ToSchema};
use uuid::Uuid;

/// Documentation for the response shown in Swagger UI.
#[derive(Debug, Serialize, ToSchema, ToResponse)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetWidgetResponse {
    pub id: Uuid,
    pub name: String,
}

/// Documentation for the request body shown in Swagger UI.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PostWidgetRequest {
    pub name: String,
}

/// Query parameters go in a separate struct deriving IntoParams.
#[derive(Debug, Deserialize, ToSchema, IntoParams)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetWidgetParams {
    pub page: Option<u32>,
}
```

Rules:

- All structs derive `Debug` and `ToSchema`.
- Response structs also derive `Serialize` and `ToResponse`.
- Request body structs also derive `Deserialize`.
- Query/path param structs also derive `IntoParams`.
- Always use `#[serde(rename_all = "camelCase")]`.
- Add `From<DomainType>` conversions here when mapping from `app_core` models.

## 3. Fill in `endpoints.rs`

The template shows a read and a write handler. Keep to its shape:

- Return `Result<Json<T>, ApiError>`, or `Result<(StatusCode, Json<T>), ApiError>` when the status
  varies, such as `201 Created`.
- Take `UserToken` when authentication is required, `Option<UserToken>` when it is optional.
- Take a read lock for reads and a write lock for writes, and **keep the lock window minimal** —
  release it before doing anything else.
- Map domain errors onto `ApiError` variants. Never leak an internal error to the caller.
- Document every response status you can return, including the failures.
- Always annotate with `#[axum_macros::debug_handler(state = AppState)]`.
- Always annotate with `#[utoipa::path(...)]` — this generates the OpenAPI spec.
- Write a doc comment above the handler; consider it as the user-facing documentation so be concise but exhaustive.
- Return `Result<Json<ResponseType>, ApiError>` for standard JSON, or `Result<(StatusCode, Json<ResponseType>), ApiError>` when the status code varies (e.g. 201 Created).
- Use `State(db): State<Arc<RwLock<dyn database::Database>>>` to access the database. Take a read lock for reads, a write lock for writes. Keep the lock window minimal.
- Use the `UserToken` extractor when authentication is mandatory; `Option<UserToken>` when optional.
- Map domain errors to `ApiError` variants — do not expose internal error details.
- Call `app_core::*` for all business logic. The handler itself should contain no domain logic.

```rust
use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use tokio::sync::RwLock;
use uuid::Uuid;

use super::models::{GetWidgetResponse, PostWidgetRequest};
use crate::{
    app_state::AppState,
    error::{ApiError, ApiErrorResponse},
    models::UserToken, // required: `UserToken`; optional: `Option<UserToken>`
};

/// Get a widget by ID.
#[axum_macros::debug_handler(state = AppState)]
#[utoipa::path(
    get,
    path = "/widget/{id}",
    params(("id" = Uuid, Path, description = "Widget ID")),
    responses(
        (status = OK, body = GetWidgetResponse, description = "The widget."),
        (status = NOT_FOUND, body = ApiErrorResponse, description = "Widget not found."),
        (status = UNAUTHORIZED, body = ApiErrorResponse, description = "Not authenticated."),
    ),
)]
pub(crate) async fn get_widget(
    _user: UserToken,
    State(db): State<Arc<RwLock<dyn database::Database>>>,
    Path(id): Path<Uuid>,
) -> Result<Json<GetWidgetResponse>, ApiError> {
    let db = db.read().await;
    let widget = app_core::widget::get_widget(&*db, id)
        .await
        .map_err(|_| ApiError::NotFound(id.to_string()))?;
    Ok(Json(GetWidgetResponse::from(widget)))
}

/// Create a new widget.
#[axum_macros::debug_handler(state = AppState)]
#[utoipa::path(
    post,
    path = "/widget",
    request_body = PostWidgetRequest,
    responses(
        (status = CREATED, body = GetWidgetResponse, description = "Widget created."),
        (status = UNAUTHORIZED, body = ApiErrorResponse, description = "Not authenticated."),
    ),
)]
pub(crate) async fn create_widget(
    _user: UserToken,
    State(db): State<Arc<RwLock<dyn database::Database>>>,
    Json(body): Json<PostWidgetRequest>,
) -> Result<(StatusCode, Json<GetWidgetResponse>), ApiError> {
    let mut db = db.write().await;
    let widget = app_core::widget::create_widget(&mut *db, body.name)
        .await
        .map_err(|e| ApiError::Unexpected(anyhow::anyhow!("{e}")))?;
    Ok((StatusCode::CREATED, Json(GetWidgetResponse::from(widget))))
}
```

## 4. Register the module

Add it to [endpoints/mod.rs](../../../crates/api/src/endpoints/mod.rs):

```rust
pub(crate) mod widget; // add this line
```

## 5. Register the routes

In [routes/router.rs](../../../crates/api/src/routes/router.rs), import each handler **and** its
`__path_` companion, which utoipa generates, then add it to `api_router`:

```rust
use crate::endpoints::widget::endpoints::{
    __path_create_widget, __path_get_widget, create_widget, get_widget,
};
```

```rust
.routes(routes!(get_widget))
.routes(routes!(create_widget, list_widgets))   // group handlers sharing one path
```

`api_router` is shared by the live server and the offline OpenAPI generator, so registering here is
what puts the endpoint in Swagger UI *and* in the frontend SDK.

## 6. Regenerate the frontend SDK

The endpoint does not exist for the frontend until the generated client is rebuilt — Skill(frontend-api-sdk).

```bash
./scripts/build_frontend_api_sdk.sh
```

## 7. Flag a breaking change

These types are the public API contract. If you **removed**, **renamed** or **changed the type of** a
field or enum key, the commit needs a `BREAKING CHANGES:` footer listing each one, prefixed `API:`.
See Skill(commit-messages).

## Checklist

```bash
./scripts/test_openapi.sh    # the committed SDK matches the router
```

- [ ] The handler calls `app_core` and holds no domain logic.
- [ ] Every response status the handler can return is documented in `utoipa::path`.
- [ ] The endpoint appears in Swagger UI at the configured path.
