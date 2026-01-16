#!/usr/bin/env bash
set -e

GIT_ROOT=$(git rev-parse --show-toplevel)
cd "$GIT_ROOT"

if [ ! -x "$(command -v sql-gen)" ]; then
    echo "Installing sql-gen"
    cargo install sql-gen
fi

if [ ! -x "$(command -v sqlx-cli)" ]; then
    echo "Installing sqlx-cli"
    cargo install sqlx-cli
fi

# TODO:
# 1. Set-up a new postgres database
#   - find an available port
#   - create a random user/password
#   - run the container
#   - wait for container to be ready
# 2. Run the migrations: sqlx migrate run --source crates/database/migrations --database_url "$DATABASE_URL" --no-dotenv
# 3. Run sql-gen: sql-gen --db-url "$DATABASE_URL" --output crates/database/src/generated_models.rs
