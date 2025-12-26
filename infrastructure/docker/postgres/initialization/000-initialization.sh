#!/usr/bin/env bash
set -euo pipefail

# Initialization script for Postgres Docker image.
# Creates the database named by DATABASE_NAME (defaults to `backend`),
# installs required extension(s), and creates two users from environment
# variables with appropriate privileges:
#   - READWRITE_USER / READWRITE_PASSWORD -> read/write (owner-like) access
#   - READONLY_USER / READONLY_PASSWORD -> read-only (SELECT) access
: "${READWRITE_USER:?Environment variable READWRITE_USER is required}"
: "${READWRITE_PASSWORD:?Environment variable READWRITE_PASSWORD is required}"
: "${READONLY_USER:?Environment variable READONLY_USER is required}"
: "${READONLY_PASSWORD:?Environment variable READONLY_PASSWORD is required}"

# Database name for the backend service
: "${DATABASE_NAME:?Environment variable DATABASE_NAME is required}"

echo "Creating database '${DATABASE_NAME}' if it does not exist..."

if ! psql --username "$POSTGRES_USER" -tAc "SELECT 1 FROM pg_database WHERE datname='${DATABASE_NAME}'" | grep -q 1; then
  psql --username "$POSTGRES_USER" -c "CREATE DATABASE ${DATABASE_NAME}"
else
  echo "Database '${DATABASE_NAME}' already exists, skipping CREATE DATABASE."
fi

echo "Installing extensions in '${DATABASE_NAME}'..."
psql --username "$POSTGRES_USER" --dbname "${DATABASE_NAME}" -v ON_ERROR_STOP=1 <<-SQL
CREATE EXTENSION IF NOT EXISTS "pg_uuidv7";
SQL

echo "Creating roles and setting privileges..."

psql --username "$POSTGRES_USER" --dbname "${DATABASE_NAME}" -v ON_ERROR_STOP=1 <<-SQL
-- Create BACKEND user if not exists
DO $$
BEGIN
   IF NOT EXISTS (SELECT FROM pg_roles WHERE rolname = '${READWRITE_USER}') THEN
      EXECUTE 'CREATE ROLE ' || quote_ident('${READWRITE_USER}') || ' WITH LOGIN PASSWORD ' || quote_literal('${READWRITE_PASSWORD}');
   END IF;
END
$$;

-- Create READ_ONLY user if not exists
DO $$
BEGIN
   IF NOT EXISTS (SELECT FROM pg_roles WHERE rolname = '${READONLY_USER}') THEN
      EXECUTE 'CREATE ROLE ' || quote_ident('${READONLY_USER}') || ' WITH LOGIN PASSWORD ' || quote_literal('${READONLY_PASSWORD}');
   END IF;
END
$$;

-- Grant backend user full access on public schema and future tables
GRANT CONNECT ON DATABASE ${DATABASE_NAME} TO ${READWRITE_USER};
GRANT USAGE ON SCHEMA public TO ${READWRITE_USER};
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO ${READWRITE_USER};
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL ON TABLES TO ${READWRITE_USER};

-- Read-only user: allow connect + select on existing and future tables
GRANT CONNECT ON DATABASE ${DATABASE_NAME} TO ${READONLY_USER};
GRANT USAGE ON SCHEMA public TO ${READONLY_USER};
GRANT SELECT ON ALL TABLES IN SCHEMA public TO ${READONLY_USER};
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT SELECT ON TABLES TO ${READONLY_USER};
SQL

echo "Initialization complete."
