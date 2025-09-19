.PHONY: build test check fmt clippy clean doc example

# Default target
all: check test

# Build the project
build:
	cargo build

# Run tests
test:
	cargo test

# Check code (build + clippy + fmt check)
check: build clippy fmt-check

# Format code
fmt:
	cargo fmt

# Check formatting
fmt-check:
	cargo fmt -- --check

# Run clippy linter
clippy:
	cargo clippy -- -D warnings

# Clean build artifacts
clean:
	cargo clean

# Generate documentation
doc:
	cargo doc --open

# Run example
example:
	cargo run --example basic_usage --features examples

# Release build
release:
	cargo build --release