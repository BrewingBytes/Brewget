#!/bin/bash
set -e

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
    -- Create databases if they don't exist
    SELECT 'CREATE DATABASE brewget_auth'
    WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = 'brewget_auth')\gexec

    SELECT 'CREATE DATABASE brewget_settings'
    WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = 'brewget_settings')\gexec
EOSQL

echo "Databases created successfully"
