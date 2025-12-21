#!/usr/bin/env bash
set -euo pipefail

# Initialization script for Postgres Docker image.
# Creates the database named by DATABASE_NAME (defaults to `backend`),
# installs required extension(s), and creates two users from environment
# variables with appropriate privileges:
#   - BACKEND_USER / BACKEND_PASSWORD -> read/write (owner-like) access
#   - READ_ONLY_USER / READ_ONLY_PASSWORD -> read-only (SELECT) access
: "${BACKEND_USER:?Environment variable BACKEND_USER is required}"
: "${BACKEND_PASSWORD:?Environment variable BACKEND_PASSWORD is required}"
: "${READ_ONLY_USER:?Environment variable READ_ONLY_USER is required}"
: "${READ_ONLY_PASSWORD:?Environment variable READ_ONLY_PASSWORD is required}"

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
   IF NOT EXISTS (SELECT FROM pg_roles WHERE rolname = '${BACKEND_USER}') THEN
      EXECUTE 'CREATE ROLE ' || quote_ident('${BACKEND_USER}') || ' WITH LOGIN PASSWORD ' || quote_literal('${BACKEND_PASSWORD}');
   END IF;
END
$$;

-- Create READ_ONLY user if not exists
DO $$
BEGIN
   IF NOT EXISTS (SELECT FROM pg_roles WHERE rolname = '${READ_ONLY_USER}') THEN
      EXECUTE 'CREATE ROLE ' || quote_ident('${READ_ONLY_USER}') || ' WITH LOGIN PASSWORD ' || quote_literal('${READ_ONLY_PASSWORD}');
   END IF;
END
$$;

-- Grant backend user full access on public schema and future tables
GRANT CONNECT ON DATABASE ${DATABASE_NAME} TO ${BACKEND_USER};
GRANT USAGE ON SCHEMA public TO ${BACKEND_USER};
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO ${BACKEND_USER};
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL ON TABLES TO ${BACKEND_USER};

-- Read-only user: allow connect + select on existing and future tables
GRANT CONNECT ON DATABASE ${DATABASE_NAME} TO ${READ_ONLY_USER};
GRANT USAGE ON SCHEMA public TO ${READ_ONLY_USER};
GRANT SELECT ON ALL TABLES IN SCHEMA public TO ${READ_ONLY_USER};
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT SELECT ON TABLES TO ${READ_ONLY_USER};
SQL

echo "Initialization complete."
