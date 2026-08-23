# Models

The domain structs shared between layers: `ApiKey`, `User`, `UserInvoiceInfo`. Plain data,
no trait, no I/O. See [crates/README.md](../README.md) for where this crate sits in the
layer stack.

Other layers convert to and from these types with `From`/`Into` rather than importing a
database or API row type directly. For example, [app_core](../app_core) converts a
`database::models::ApiKey` into a `models::ApiKey` in
[`app_core::models::api_key_from_db`](../app_core/src/models.rs).

## Structure

One file per struct, named after it in snake_case.

```txt
models/
└── src/
    ├── api_key.rs           # ApiKey
    ├── user.rs              # User
    └── user_invoice_info.rs # UserInvoiceInfo
```
