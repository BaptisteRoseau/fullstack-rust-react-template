#!/usr/bin/env bash
set -e

GIT_ROOT=$(git rev-parse --show-toplevel)
cd "$GIT_ROOT"

function ensure_installed(){
    executable=$1
    if [ ! -x "$(command -v $executable)" ]; then
        echo "Missing $executable, please install it and rerun this script" >&2
        exit 1
    fi
}

ensure_installed "docker"
ensure_installed "sqlx"
ensure_installed "cargo"

source .env
POSTGRES_PORT=5678

# Add a CLI argument to select the output file of this script
# Prefix the generated file with a comment

echo "Starting a new database instance"
docker stop sqlx_query_temp || echo ""
docker run \
    -e POSTGRES_DB=${POSTGRES_DATABASE} \
    -e POSTGRES_USER=${POSTGRES_USER} \
    -e POSTGRES_PASSWORD=${POSTGRES_PASSWORD} \
    -e READWRITE_USER=${POSTGRES_READWRITE_USER} \
    -e READWRITE_PASSWORD=${POSTGRES_READWRITE_PASSWORD} \
    -e READONLY_USER=${POSTGRES_READONLY_USER} \
    -e READONLY_PASSWORD=${POSTGRES_READONLY_PASSWORD} \
    -p $POSTGRES_PORT:5432 \
    --rm \
    --detach \
    --name sqlx_query_temp \
    app_postgres:latest > /dev/null

echo "Waiting for the database to accept connections"
for _ in $(seq 60); do
    if docker exec sqlx_query_temp pg_isready -q \
        -h 127.0.0.1 -p 5432 -U "${POSTGRES_USER}" -d "${POSTGRES_DATABASE}"; then
        break
    fi
    sleep 1
done
if ! docker exec sqlx_query_temp pg_isready -q \
    -h 127.0.0.1 -p 5432 -U "${POSTGRES_USER}" -d "${POSTGRES_DATABASE}"; then
    echo "The database never became ready" >&2
    docker stop sqlx_query_temp >/dev/null 2>&1 || true
    exit 1
fi

echo "Starting SQLx migrations"
DATABASE_URL="postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@127.0.0.1:${POSTGRES_PORT}/${POSTGRES_DATABASE}?sslmode=disable"
sqlx migrate run  --no-dotenv --database-url "$DATABASE_URL" --source "$GIT_ROOT/crates/database/migrations"

echo "Writing SQLx query metadata"
cargo --quiet sqlx prepare --no-dotenv --database-url "$DATABASE_URL" --workspace

docker stop sqlx_query_temp
