.PHONY: help build test lint fmt clean run install check release

# Default target
help:
	@echo "Available targets:"
	@echo "  build       - Build the project in debug mode"
	@echo "  release     - Build the project in release mode"
	@echo "  test        - Run all tests"
	@echo "  lint        - Run clippy linter"
	@echo "  fmt         - Format code with rustfmt"
	@echo "  check       - Run fmt and lint checks"
	@echo "  clean       - Clean build artifacts"
	@echo "  run         - Run the provider locally"
	@echo "  install     - Install wash CLI"
	@echo "  all         - Run fmt, lint, test, and build"

# Build targets
build:
	cargo build

release:
	cargo build --release

# Testing
test:
	cargo test

test-verbose:
	cargo test -- --nocapture

# Code quality
lint:
	cargo clippy -- -D warnings

fmt:
	cargo fmt

fmt-check:
	cargo fmt --all -- --check

check: fmt-check lint

# Cleanup
clean:
	cargo clean

# Run locally (requires NATS server)
run: build
	@echo "Note: Ensure NATS server is running (default: nats://localhost:4222)"
	@echo "Set NATS_URL environment variable if using a different address"
	./target/debug/wasmcloud-websocket-provider

run-release: release
	./target/release/wasmcloud-websocket-provider

# Install wash CLI
install-wash:
	@echo "Installing wash CLI..."
	curl -fsSL https://raw.githubusercontent.com/wasmcloud/wash/refs/heads/main/install.sh | bash
	@echo "Add wash to your PATH: export PATH=\"\$$PATH:\$$HOME/.wash/bin\""

# Development workflow
all: fmt lint test build
	@echo "✓ All checks passed!"

# Watch mode for development (requires cargo-watch)
watch:
	cargo watch -x check -x test -x build

install-watch:
	cargo install cargo-watch

# CI simulation
ci: check test build
	@echo "✓ CI checks passed!"

# Security audit (requires cargo-audit)
audit:
	cargo audit

install-audit:
	cargo install cargo-audit
