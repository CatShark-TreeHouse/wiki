# CatShark TreeHouse wiki task runner. Run `just` to list recipes.

frontend := justfile_directory() / "frontend" / "short-shepherd"

# List available recipes.
default:
    @just --list

# Install frontend dependencies.
install:
    cd {{frontend}} && npm install

# Run the Astro dev server.
dev:
    cd {{frontend}} && npm run dev

# Build the static site.
build:
    cd {{frontend}} && npm run build

# Run what CI runs: format check, astro check, build.
check:
    cd {{frontend}} && npm run format:check && npm run check && npm run build

# Format the frontend.
fmt:
    cd {{frontend}} && npm run format
