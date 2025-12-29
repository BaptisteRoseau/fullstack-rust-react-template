#!/usr/bin/env bash
set -euo pipefail

# Initialization script for Postgres Docker image.
# Creates the database named by POSTGRES_DB (defaults to `backend`),
# installs required extension(s), and creates two users from environment
# variables with appropriate privileges:
#   - READWRITE_USER / READWRITE_PASSWORD -> read/write (owner-like) access
#   - READONLY_USER / READONLY_PASSWORD -> read-only (SELECT) access
: "${READWRITE_USER:?Environment variable READWRITE_USER is required}"
: "${READWRITE_PASSWORD:?Environment variable READWRITE_PASSWORD is required}"
: "${READONLY_USER:?Environment variable READONLY_USER is required}"
: "${READONLY_PASSWORD:?Environment variable READONLY_PASSWORD is required}"

# Database name for the backend service
: "${POSTGRES_DB:?Environment variable POSTGRES_DB is required}"

echo "Creating database '${POSTGRES_DB}' if it does not exist..."

if ! psql --username "$POSTGRES_USER" -tAc "SELECT 1 FROM pg_database WHERE datname='${POSTGRES_DB}'" | grep -q 1; then
  psql --username "$POSTGRES_USER" -c "CREATE DATABASE ${POSTGRES_DB}"
else
  echo "Database '${POSTGRES_DB}' already exists, skipping CREATE DATABASE."
fi

echo "Creating '${READWRITE_USER}' and '${READONLY_USER}' users and setting privileges..."

psql --username "$POSTGRES_USER" --dbname "${POSTGRES_DB}" -v ON_ERROR_STOP=1 <<-SQL
-- Create the roles & users
CREATE ROLE ${READWRITE_USER} WITH LOGIN PASSWORD '${READWRITE_PASSWORD}';
CREATE ROLE ${READONLY_USER} WITH LOGIN PASSWORD '${READONLY_PASSWORD}';

-- Grant read-write user full access on public schema and future tables
GRANT CONNECT ON DATABASE ${POSTGRES_DB} TO ${READWRITE_USER};
GRANT USAGE ON SCHEMA public TO ${READWRITE_USER};
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO ${READWRITE_USER};

-- Read-only user: allow connect + select on existing and future tables
GRANT CONNECT ON DATABASE ${POSTGRES_DB} TO ${READONLY_USER};
GRANT USAGE ON SCHEMA public TO ${READONLY_USER};
GRANT SELECT ON ALL TABLES IN SCHEMA public TO ${READONLY_USER};
SQL

echo "Initialization complete."
