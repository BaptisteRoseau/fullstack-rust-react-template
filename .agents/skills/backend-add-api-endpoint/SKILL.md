---
name: backend-add-api-endpoint
description: Use this skill when adding a new HTTP endpoint to the Rust API (`crates/api`).
---

# Skill: Add a Backend API Endpoint

Use this skill when adding a new HTTP endpoint to the Rust API (`crates/api`).

## Scope rule

The `api` crate is responsible only for HTTP concerns: extracting inputs, validating shape, calling `app_core`, and serialising the response. **Business logic must live in `crates/app_core`, not here.** If you find yourself writing domain logic inside an endpoint handler, stop and move it to `app_core` first.

---

## 1. Create the endpoint directory

Each endpoint group lives under `crates/api/src/endpoints/<name>/` with three files:

```
crates/api/src/endpoints/<name>/
├── models.rs     # HTTP request/response types
├── endpoints.rs  # Handler functions
└── mod.rs
```

For nested paths like `/user/resources`, use `__` as the path separator:

```
crates/api/src/endpoints/user__resources/
```

---

## 2. Write `models.rs`

Define one struct per request body or response body. Naming convention: `<Method><Resource><"Request"|"Response">`.

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

---

## 3. Write `endpoints.rs`

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

Rules:
- Always annotate with `#[axum_macros::debug_handler(state = AppState)]`.
- Always annotate with `#[utoipa::path(...)]` — this generates the OpenAPI spec.
- Write a doc comment above the handler; consider it as the user-facing documentation so be concise but exhaustive.
- Return `Result<Json<ResponseType>, ApiError>` for standard JSON, or `Result<(StatusCode, Json<ResponseType>), ApiError>` when the status code varies (e.g. 201 Created).
- Use `State(db): State<Arc<RwLock<dyn database::Database>>>` to access the database. Take a read lock for reads, a write lock for writes. Keep the lock window minimal.
- Use the `UserToken` extractor when authentication is mandatory; `Option<UserToken>` when optional.
- Map domain errors to `ApiError` variants — do not expose internal error details.
- Call `app_core::*` for all business logic. The handler itself should contain no domain logic.

---

## 4. Write `mod.rs`

```rust
pub mod endpoints;
pub mod models;
```

---

## 5. Register in the endpoints module

Add the new module to `crates/api/src/endpoints/mod.rs`:

```rust
pub(crate) mod widget; // add this line
```

---

## 6. Register in the router

Edit `crates/api/src/routes.rs`:

**Import the handler and its `__path_` companion** (generated by utoipa):

```rust
use crate::endpoints::widget::endpoints::{
    __path_create_widget, __path_get_widget,
    create_widget, get_widget,
};
```

**Add `.routes(routes!(...))` inside `public_routes`**:

```rust
.routes(routes!(get_widget))
.routes(routes!(create_widget))
```

---

## 7. Verify

```bash
cargo build -p api
cargo clippy -p api
cargo test -p api
```

The new endpoint will appear automatically in the Swagger UI at the path configured in `config.swagger.swagger_ui_path`.
