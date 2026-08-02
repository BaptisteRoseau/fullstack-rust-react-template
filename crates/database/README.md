# Database

Here lies the database interaction layer of our backend.

It uses [`sqlx`](https://docs.rs/crate/sqlx/latest) and [`sqlx-cli`](https://docs.rs/crate/sqlx-cli/latest) to make migrations and interact with the database.

## Quick Start

Install `sqlx` using the following:

```cmd
cargo install sqlx-cli
```

Migrations are located under [migrations](./migrations/). To create a new migration, run the following command from the current

```cmd
sqlx migrate add -rs <name>
```

Always provide a "rollback" migration under the _down.sql_ file. Rollbacks are supposed to be performed automatically after an unsuccessful migration, even if the rollback is empty.

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

To generate it, run or read the script [build_database_rust_models.sh](./scripts/generate_database_rust_models.sh).
