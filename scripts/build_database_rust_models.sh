#!/usr/bin/env bash
set -e

GIT_ROOT=$(git rev-parse --show-toplevel)
cd "$GIT_ROOT"

function ensure_installed(){
    executable=$1
    if [ ! -x "$(command -v $executable)" ]; then
        echo "Missing $executable, please install it and rerun this script" >2
        exit 1
    fi
}

ensure_installed "sql-gen"
ensure_installed "sqlx"
ensure_installed "docker"

POSTGRES_USER=root

# TODO:
# 1. Set-up a new postgres database
#   - find an available port
#   - create a random user/password
#   - run the container
#   - wait for container to be ready

echo "Starting a new database instance"

DATABASE_URL="postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@postgres:5432/${POSTGRES_DATABASE}?sslmode=disable"

echo "Starting SQLx migrations"
sqlx migrate run --source "$GIT_ROOT/crates/database/migrations" --database_url "$DATABASE_URL" --no-dotenv

echo "Generating models"
sql-gen --db-url "$DATABASE_URL" --output crates/database/src/generated_models.rs
