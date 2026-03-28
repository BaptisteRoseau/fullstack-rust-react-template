# Endpoints

In this directoy are defined the API endpoints.

## Organization

Each endpoint should be defined under a directory as follows:

```txt
<name>
├── models.rs # The API responses & params
├── endpoints.rs # The API functions
└── mod.rs
```

## Models

Models are the specific inputs and outputs of an API.

Each model should derive from `Debug` and `utoipa::ToSchema`. It is possible to add `From<T>` when necessary in an endpoint.

Parameters should derive from `Deserialize` and `IntoParams`.
Responses should derive from `Serialize` and `ToResponse`.

They are named as `<method><structure><"Response"|"Params">`.

For example, for `GET /user/:path`, there is no params but the response model would be `GetUserResponse`.

For example:

```rust
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToResponse, ToSchema};
use uuid::Uuid;

/// <Here should be the documentation of the response>
#[derive(Debug, Serialize, ToSchema, ToResponse)]
pub(crate) struct GetUserResponse {
    pub name: String,
}

impl From<String> for GetUserResponse {
    fn from(value: String) -> Self {
        Self { name: value }
    }
}

/// <Here should be the documentation of the parameter>
#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub(crate) struct PostUserParams {
    pub id: Uuid,
}
```

## Endpoints

Endpoints are defined using the utoipa crate as follows:

```rust
use super::models::GetUserResponse;
use crate::error::ApiError;

/// <Here is the documentation of the endpoint>
#[utoipa::path(
    get,
    path = "/user/:uuid",
    responses(
        (status = OK, body = GetUserResponse, description = "The user information."),
        (status = NOT_FOUND, body = ApiError, description = "The user does not exist."),
    ),
    params(
        ("uuid" = Uuid, Path, description = "The user uuid"),
    )
)]
pub(crate) async fn get_user(
    uuid: Path<Uuid>,
    opt_user: OptionalUser,
    State(state): State<AppState>,
) -> Result<Json<GetUserResponse>, ApiError> {
    todo!()
}
```

Make sure to:

- Return a `Result<Json<TheCorrespondingResponseModel>, ApiError>`
- Document the API using a docstring, this will be used to generate user documentation.
- Document the parameters and responses
- Use the correct HTTP method
- Use `State(state): State<AppState>` when accessing the state is required
- Use a minimal windows when using a lock from an object in the state
- Implement the actual logic from in the `crates/app_core` crate, the `api` only handles the input/output/state management

## Mod

Should always be:

```rust
pub mod models;
pub mod endpoints;
```
