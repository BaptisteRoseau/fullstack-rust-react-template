# Database CRUD Derive

A proc-macro crate. It exposes one derive, `#[derive(Crud)]`, used by
[database](../database) on its row structs to generate their CRUD queries instead of
hand-writing them.

Only [database](../database) depends on this crate. The generated code calls
`crate::crud::{CrudExecutor, CrudValue, CrudError}`, so it only compiles inside a crate
that defines that module — currently only `database` does.

## What it generates

For a struct with an `id` field (required) plus any number of other fields:

- `create(db, <user fields>)` — inserts a row, returns the struct.
- `read_by_<field>` — one per field. `read_by_id` returns a single row; every other
  `read_by_<field>` returns a `Vec`.
- `delete(db, id)` — deletes by id, returns the affected row count.
- `<Struct>Patch` — a companion struct with every user field wrapped in `Option`
  (`Option<Option<T>>` for a field that is itself an `Option<T>`, to distinguish "leave
  alone" from "set to null"), plus `set_<field>` builder methods and a `patch` method on
  the original struct.

`id`, `created_at` and `updated_at` are treated as automatic: never part of the generated
`create` parameters or the patch struct.

Supported field types are `String`, `Uuid`, `DateTime`, `bool`, `i32`, `i64`, `f64` and
`serde_json::Value`, plus `Option<...>` of each (`Option<Uuid>` is not yet supported).
An unsupported type is a compile error, not a panic.

## Directory

```txt
database_crud_derive/
├── src/
│   ├── lib.rs           # the #[proc_macro_derive(Crud)] entry point
│   ├── parse.rs         # DeriveInput -> ModelInfo (struct name, table name, fields)
│   ├── table_name.rs    # struct name -> plural snake_case table name
│   ├── type_mapping.rs  # Rust type -> CrudValue variant and parameter type
│   ├── gen_create.rs
│   ├── gen_read.rs
│   ├── gen_delete.rs
│   └── gen_patch.rs
└── tests/
    ├── trybuild.rs      # compile_fail cases
    └── fixtures/        # one .rs / .stderr pair per rejected input
```
