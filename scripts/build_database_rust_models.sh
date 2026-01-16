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

# Run this sqlx offline script then this one

echo "Generating models"
sql-gen --db-url "$DATABASE_URL" --output crates/database/src/generated_models.rs
