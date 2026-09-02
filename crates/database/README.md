# Database

Here lies the database interaction layer of our backend.

It uses [`sqlx`](https://docs.rs/crate/sqlx/latest) for queries and
[`sqlx-cli`](https://docs.rs/crate/sqlx-cli/latest) for migrations.

Migrations live in [migrations](./migrations/), numbered and applied in order. Each one is a
reversible pair: a `.up.sql` and a `.down.sql`. A committed migration is never edited.

## Conventions

### Field naming

Every table's primary key is the `id` field as follows:

```sql
id UUID UNIQUE NOT NULL DEFAULT uuidv7(),
-- ...
PRIMARY KEY(id),
```

### Created At & Updated At

By default, every table has the following fields that are added through a trigger upon table creation:

```sql
created_at TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL,
updated_at TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL,
```

Those fields are updated by default by Postgres so don't bother setting them manually.

### Database trait

The `Database` trait is split into per-entity traits (e.g. `DatabaseUser`, `DatabaseApiKey`) under [`src/database/`](./src/database/), one file each, since a single trait covering every entity would be too large to stay readable.

`Database` itself is just `DatabaseApiKey + DatabaseUser + ...` with a blanket impl, so any backend implementing all the entity traits gets `Database` for free — no manual impl needed.

### Generate Models

The [`generated_models.rs`](./src/generated_models.rs) file is generated using `sql-gen`. It contains structures used to convert data from Postgres to Rust and vice-versa.

DO NOT manually modify this file. If you need to create other models use [./src/models.rs](./src/models.rs).

To generate it, run or read the script [build_database_rust_models.sh](../../scripts/build_database_rust_models.sh).

Not every table goes through the derive. `database_crud_derive::Crud` cannot express an
`Option<Uuid>` column, so the `directories`, `files`, `directory_permissions` and
`file_permissions` tables — all of which carry a nullable `parent_id` or need tree queries
the derive cannot write — have hand-written row structs in
[`src/models.rs`](./src/models.rs) and hand-written queries in
[`src/backends/postgres.rs`](./src/backends/postgres.rs).

## Skills

- [backend-database-migration](../../.claude/skills/backend-database-migration/SKILL.md)
- [backend-trait-test](../../.claude/skills/backend-trait-test/SKILL.md)
- [backend-feature-gating](../../.claude/skills/backend-feature-gating/SKILL.md)
