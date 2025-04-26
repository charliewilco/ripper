# Contributing to Ripper

Thank you for considering contributing to Ripper! This document provides guidelines and instructions for contributing.

## Code of Conduct

Please be respectful and considerate of others when contributing to this project. We strive to maintain a welcoming and inclusive environment for everyone.

## How to Contribute

### Reporting Bugs

If you find a bug, please create an issue with the following information:

1. A clear, descriptive title
2. Steps to reproduce the bug
3. Expected behavior
4. Actual behavior
5. Any relevant logs or output
6. Your environment (OS, Rust version, etc.)

### Suggesting Features

Feature suggestions are welcome! Please create an issue with:

1. A clear description of the feature
2. The problem it solves
3. Any implementation ideas you have

### Pull Requests

1. Fork the repository
2. Create a new branch: `git checkout -b feature-branch-name`
3. Make your changes
4. Run the tests: `cargo test`
5. Run the formatter: `cargo fmt`
6. Run the linter: `cargo clippy`
7. Commit your changes: `git commit -m "Description of changes"`
8. Push to your fork: `git push origin feature-branch-name`
9. Create a pull request

## Development Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/charliewilco/ripper.git
   cd ripper
   ```

2. Install dependencies:
   ```bash
   # Install Rust (if not already installed)
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # Install development tools
   rustup component add rustfmt clippy
   ```

3. Build the project:
   ```bash
   cargo build
   ```

4. Run tests:
   ```bash
   cargo test
   ```

## Testing

See [docs/TESTING.md](docs/TESTING.md) for detailed information about our testing strategy.

## Coding Style

We follow standard Rust style guidelines:

1. Use `rustfmt` to format your code
2. Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
3. Document public API with rustdoc comments
4. Write tests for all new functionality

## Git Workflow

1. Keep commits small and focused on a single change
2. Write clear commit messages that explain why the change was made
3. Rebase your branch before submitting a pull request
4. Prefer squashing fixup commits before the final PR

## Release Process

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Commit these changes: `git commit -m "Bump version to x.y.z"`
4. Tag the release: `git tag vx.y.z`
5. Push tags: `git push && git push --tags`

## License

By contributing to this project, you agree that your contributions will be licensed under the project's [MIT License](LICENSE).

Thank you for your contributions!
