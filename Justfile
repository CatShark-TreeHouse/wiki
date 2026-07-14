# Catshark wiki — task runner. Run `just` to list recipes.

# SQLite lives as a single file in backend/. `mode=rwc` creates it on first run.
export DATABASE_URL := env_var_or_default("DATABASE_URL", "sqlite://" + justfile_directory() + "/backend/wiki.db?mode=rwc")

backend := justfile_directory() / "backend"
frontend := justfile_directory() / "frontend" / "short-shepherd"

# List available recipes.
default:
    @just --list

# --- backend -------------------------------------------------------------

# Build the whole Rust workspace.
build:
    cd {{backend}} && cargo build

# Run all backend tests.
test:
    cd {{backend}} && cargo test

# Run the HTTP API (and the Telegram bot too, if TELOXIDE_TOKEN is set).
run-api:
    cd {{backend}} && cargo run -p api

# Run with the bot enabled (requires a bot token).
run-bot token:
    cd {{backend}} && TELOXIDE_TOKEN={{token}} cargo run -p api

# Delete the local SQLite database file.
db-reset:
    rm -f {{backend}}/wiki.db {{backend}}/wiki.db-shm {{backend}}/wiki.db-wal

# Seed the DB with the legacy banned/spoilered lists (idempotent).
db-seed:
    sqlite3 {{backend}}/wiki.db < {{backend}}/seed.sql

# --- frontend ------------------------------------------------------------

# Install frontend dependencies.
fe-install:
    cd {{frontend}} && npm install

# Run the Astro dev server (expects the API on :8080).
fe-dev:
    cd {{frontend}} && npm run dev

# Build the static frontend.
fe-build:
    cd {{frontend}} && npm run build

# --- combined ------------------------------------------------------------

# Run the API and the frontend dev server together.
dev:
    cd {{backend}} && cargo run -p api &
    cd {{frontend}} && npm run dev
