#!/usr/bin/env sh
pg_isready --quiet -U "${POSTGRES_USER}"