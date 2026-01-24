# Repository Guidelines

## Project Structure & Module Organization
- `src/`: Rust library and CLI entry points (`src/lib.rs`, `src/main.rs`).
- `tests/`: Integration and CLI tests (`tests/integration_test.rs`, `tests/cli_test.rs`).
- `benches/`: Benchmarks (`benches/benchmark.rs`).
- `docs/`: Project docs, including testing strategy (`docs/TESTING.md`).
- Root config: `Cargo.toml`, `justfile`, `lefthook.toml`.

## Build, Test, and Development Commands
Preferred runner is `just` (see `justfile`). Examples:
- `just build` or `cargo build`: Build debug binary.
- `just build-release` or `cargo build --release`: Optimized build.
- `just run -- <args>`: Run the CLI via Cargo.
- `just test`: Run the full test suite.
- `just lint`: Run clippy with warnings as errors.
- `just format`: Apply rustfmt to all crates.
- `just bench`: Run benchmarks.

## Coding Style & Naming Conventions
- Indentation: hard tabs (see `.rustfmt.toml`).
- Format with `cargo fmt --all` and lint with `cargo clippy -- -D warnings`.
- Follow Rust API Guidelines for public APIs and add rustdoc comments where relevant.
- Test files are named by scope, e.g. `tests/integration_test.rs`, `tests/cli_test.rs`.

## Testing Guidelines
- Unit tests live in `src/lib.rs` under `mod tests`.
- Integration tests: `cargo test --test integration_test`.
- CLI tests (assert_cmd): `cargo test --test cli_test`.
- Coverage is optional; see `docs/TESTING.md` for tarpaulin/grcov examples.

## Commit & Pull Request Guidelines
- Commit messages in history are short, imperative, and capitalized (e.g. "Add ...", "Fix ...", "Update ...").
- Keep commits focused; squash fixups before final PR when possible.
- Before opening a PR: run `cargo test`, `cargo fmt`, and `cargo clippy`.
- PRs should include a clear description and, when applicable, steps to reproduce or verify changes.

## Tooling Notes
- `lefthook` can enforce formatting, linting, and tests via git hooks (see `lefthook.toml`).
- Optional safety checks are defined but skipped by default (cargo-audit, cargo-udeps).
