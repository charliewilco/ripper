# Testing Strategy for Ripper

This document outlines the testing approach for the Ripper project.

## Testing Levels

### 1. Unit Tests

Unit tests are located in the `src/lib.rs` file inside the `mod tests` section. These tests focus on the individual components of the application:

- `FoundFile` struct and its methods
- `find_files` function
- `delete_files` function

Unit tests use `tempfile` to create temporary directories and files for testing, ensuring they can be executed in any environment and will clean up after themselves.

Run with:
```bash
cargo test --lib
```

### 2. Integration Tests

Integration tests are located in the `tests/integration_test.rs` file. These tests verify that the components work together correctly:

- Finding files with various patterns
- Deleting files after finding them
- Error handling for various edge cases

These tests also use `tempfile` for creating isolated test environments.

Run with:
```bash
cargo test --test integration_test
```

### 3. CLI Tests

CLI tests in `tests/cli_test.rs` test the command-line interface using the `assert_cmd` crate, which allows testing the full application as a black box:

- Command-line argument parsing
- Read-only `find` behavior
- Destructive `delete` behavior, including prompt and `--yes`
- Error handling for incorrect arguments and invalid regex patterns
- Various flag combinations (`--verbose`, `--follow-links`, `--yes`)
- Exit codes and output validation

Run with:
```bash
cargo test --test cli_test
```

### 4. Performance Benchmarks

Benchmarks in `benches/benchmark.rs` use Criterion to measure the performance of key operations:

- Finding files with simple patterns
- Finding files with complex patterns
- File deletion performance
- End-to-end operation performance
- Performance in nested directory structures

Run with:
```bash
cargo bench
```

## Test Coverage

To generate test coverage reports, you can use `grcov` or `tarpaulin`:

```bash
# Using tarpaulin
cargo install cargo-tarpaulin
cargo tarpaulin --out Html

# Using grcov (requires nightly)
cargo install grcov
export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off"
cargo +nightly test
grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./coverage/
```

## Continuous Integration

Our GitHub Actions workflow (`.github/workflows/build.yml`) automates testing:

- Unit, integration, and CLI tests are run on every push and PR
- Tests are run on multiple platforms (Linux, macOS, Windows)
- Code formatting is checked with `rustfmt`
- Code quality is enforced with `clippy`
- Benchmarks are compiled on stable with `cargo bench --no-run`

## Test-Driven Development

We follow TDD principles for development:

1. Write a failing test for new functionality
2. Implement the minimum code needed to make the test pass
3. Refactor the code while keeping tests green
4. Repeat for each new feature or bug fix

## Manual Testing Checklist

Before major releases, perform these manual tests:

- [ ] Preview `.DS_Store` files in a test directory with `ripper find`
- [ ] Delete `.DS_Store` files in a test directory with `ripper delete`
- [ ] Verify verbose output contains all expected information
- [ ] Verify symlinked directories are ignored unless `--follow-links` is provided
- [ ] Test with very large directories (1000+ files)
- [ ] Test with complex regex patterns
- [ ] Verify behavior when no matching files are found
- [ ] Verify error messages when files cannot be deleted
