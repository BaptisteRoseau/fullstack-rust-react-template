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
    opt_user: Option<UserToken>, // <-- An extractor allowing to get an Option<UserToken> from the request parts.
    State(_state): State<AppState>,
) -> Result<Json<GetUserResponse>, ApiError> {
    todo!();
}
```

Implement new extractors here when you need to extract data from the HTTP request content.

They can also be used to directly return an error if the extraction is unsuccessful, acting like a middleware.

The `Rejection` type should always be `ApiError`.

## Implementation: optional extractors

When extraction can legitimately yield "nothing" (e.g. an anonymous request has no
`UserToken`), the extractor is optional. In that case:

- Implement `OptionalFromRequestParts` and put the **core extraction logic there**. Return
  `Ok(None)` when the value is simply absent, and `Err(ApiError)` only when the request is
  malformed (e.g. a present but invalid header).
- Implement `FromRequestParts` as a thin wrapper that delegates to the optional impl and
  turns `None` into a rejection. This is the "required" form of the extractor.

This gives handlers two usages for free:

- `Option<UserToken>` — present if extractable, `None` otherwise (uses the
  `OptionalFromRequestParts` impl directly, via Axum's blanket
  `impl FromRequestParts for Option<T> where T: OptionalFromRequestParts`).
- `UserToken` — required; the handler is never reached if extraction fails (uses the
  `FromRequestParts` impl).

> Note: Axum does **not** derive `FromRequestParts` from `OptionalFromRequestParts`, and a
> generic blanket impl is impossible (orphan rules + coherence with Axum's own extractors).
> The required form must be written per type, as below.

- Make a new file in this directory named after the extracted struct in snake case (here `user.rs`).
- Implement `OptionalFromRequestParts` with the extraction logic, then `FromRequestParts`
  delegating to it.

```rs
impl<S> FromRequestParts<S> for UserToken
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match <Option<Self>>::from_request_parts(parts, state).await? {
            Some(user) => Ok(user),
            None => Err(ApiError::from(ExtractorError::NotLoggedIn)),
        }
    }
}

impl<S> OptionalFromRequestParts<S> for UserToken
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Option<Self>, Self::Rejection> {
        let headers = HeaderMap::from_request_parts(parts, state)
            .await
            .map_err(anyhow::Error::from)?;
        let header = match headers.get(header::AUTHORIZATION) {
            Some(header) => header,
            None => {
                debug!("Anonymous user");
                return Ok(None);
            }
        };

        todo!();
    }
}
```

See `user.rs` for the full reference implementation.

## Implementation: non-optional extractors

When extraction without a value makes no sense (e.g. `AppState`, which is always present),
do **not** implement `OptionalFromRequestParts`. Implement only `FromRequestParts`.

- Make a new file in this directory named after the extracted struct in snake case (here `app_state.rs`).
- Implement `FromRequestParts` directly.

```rs
impl<S> FromRequestParts<S> for AppState
where
    Self: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Ok(Self::from_ref(state))
    }
}
```

See `app_state.rs` for the full reference implementation.
