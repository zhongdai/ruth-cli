# ruth-cli task runner

# Show available recipes
default:
    @just --list

# Build in debug mode
build:
    cargo build

# Build in release mode
build-release:
    cargo build --release

# Run all tests
test:
    cargo test

# Run clippy lints
lint:
    cargo clippy -- -D warnings

# Format code
fmt:
    cargo fmt

# Check formatting without modifying
fmt-check:
    cargo fmt --check

# Run all checks (test + lint + fmt)
check: test lint fmt-check

# Create a new release tag and push it (triggers GitHub Actions)
# Usage: just release 0.1.0
release version:
    #!/usr/bin/env bash
    set -euo pipefail
    TAG="v{{version}}"
    if git rev-parse "$TAG" >/dev/null 2>&1; then
        echo "Error: tag $TAG already exists"
        exit 1
    fi
    echo "Running checks before release..."
    just check
    echo ""
    echo "Creating tag $TAG..."
    git tag -a "$TAG" -m "Release $TAG"
    echo "Pushing tag $TAG to origin..."
    git push origin "$TAG"
    echo ""
    echo "Done! Release $TAG pushed. GitHub Actions will build and publish artifacts."
    echo "Track progress: https://github.com/zhongdai/ruth-cli/actions"

# List all tags
tags:
    git tag -l --sort=-v:refname

# Install locally
install:
    cargo install --path .

# Clean build artifacts
clean:
    cargo clean
