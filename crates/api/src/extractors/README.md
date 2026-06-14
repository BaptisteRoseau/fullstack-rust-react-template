# Extractors

This directory contains definitions of Axum extractors to provide endpoints with structs extracted from types.

For example:

```rs
/// Get the information of a user.
#[axum_macros::debug_handler]
#[utoipa::path(
    get,
    path = "/user/{uuid}",
    ...
)]
pub(crate) async fn get_user(
    _uuid: Path<Uuid>,
    opt_user: OptionalUser, // <-- An extractor is allowing to get the OptionalUser from the request parts.
    State(_state): State<AppState>,
) -> Result<Json<GetUserResponse>, ApiError> {
    todo!();
}
```

Implement new extractors here when you need to extract data from the HTTP request content.

They can also be used to directly return an error if the extraction is unsuccessful, acting like a middleware.

## Implementations

- Make a new file in this directory name as the extractor struct in snake case (here `optional_user.rs`)
- Create the structure that will be used in the endpoints (here `OptionalUser`)
- Implement the `FromRequestParts` trait

```rs
pub struct OptionalUser(Option<UserToken>);

impl OptionalUser {
    pub fn inner(self) -> Option<UserToken> {
        self.0
    }
}

impl<S> FromRimpl<S> FromRequestParts<S> for OptionalUser
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let headers = HeaderMap::from_request_parts(parts, state)
            .await
            .map_err(anyhow::Error::from)?;
        let header = match headers.get(header::AUTHORIZATION) {
            Some(header) => header,
            None => {
                debug!("Anonymous user");
                return Ok(OptionalUser(None));
            }
        };

        todo!();
    }
}
```

The `Rejection` should always be `ApiError`.
