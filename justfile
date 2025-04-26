# justfile for ripper
# https://github.com/casey/just

# Default command to run
default:
	@just --list

# Build the project
build:
	cargo build

# Build with optimizations
build-release:
	cargo build --release

# Run the project
run *ARGS:
	cargo run -- {{ARGS}}

# Run with optimizations
run-release *ARGS:
	cargo run --release -- {{ARGS}}

# Format the code with rustfmt
format:
	cargo fmt --all

# Run clippy linter
lint:
	cargo clippy -- -D warnings

# Watch for changes and run tests
watch:
	cargo watch -x test

# Run all tests
test:
	cargo test

# Run unit tests only
test-unit:
	cargo test --lib

# Run integration tests
test-integration:
	cargo test --test integration_test

# Run CLI tests
test-cli:
	cargo test --test cli_test

# Run benchmarks
bench:
	cargo bench

# Clean the project
clean:
	cargo clean

# Install dependencies
deps:
	rustup component add rustfmt clippy
	cargo install cargo-watch

# Check for outdated dependencies
outdated:
	cargo outdated
