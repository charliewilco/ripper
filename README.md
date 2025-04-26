# Ripper

A command-line tool to find and delete files matching a pattern, powered by ripgrep's regex engine.

## Installation

```bash
# Clone the repository
git clone https://github.com/charliewilco/ripper.git
cd ripper

# Build the project
cargo build --release

# Optional: Install the binary
cargo install --path .
```

## Usage

```bash
# Find all .DS_Store files in the current directory
ripper find "\.DS_Store"

# Find all .DS_Store files in your home directory
ripper find "\.DS_Store" -d ~

# Find and automatically delete all .DS_Store files in your home directory
ripper find "\.DS_Store" -d ~ -y

# Find with verbose output
ripper find "\.DS_Store" -d ~ -v
```

## Examples

**Finding and deleting `.DS_Store` files:**

```bash
# From your home directory
ripper find "\.DS_Store" -d ~
```

**Finding and deleting temporary files:**

```bash
# Find all .tmp files
ripper find "\.tmp$" -d /path/to/project
```

**Finding log files over 7 days old:**

```bash
# Use with other tools like find
find /var/log -type f -name "*.log" -mtime +7 | xargs -I {} bash -c 'ripper find "$(basename {})" -d "$(dirname {})" -y'
```

## Options

- `-d, --dir <DIR>` - Directory to search in (defaults to current directory)
- `-y, --yes` - Automatically confirm deletion without prompting
- `-v, --verbose` - Show verbose output

## Development

### Using Just Commands

This project uses [just](https://github.com/casey/just) as a command runner. Install it with:

```bash
cargo install just
```

Available commands:

```bash
# List all available commands
just

# Build the project
just build

# Run the linter
just lint

# Format the code
just format

# Watch for changes and run tests
just watch

# Run all tests
just test
```

### Using Lefthook

This project uses [lefthook](https://github.com/evilmartians/lefthook) for git hooks. Install it with:

```bash
# Using cargo
cargo install lefthook

# Or using homebrew
brew install lefthook
```

Then, initialize it in your local repository:

```bash
# Initialize lefthook
lefthook install

# Run a specific hook
lefthook run pre-commit

# Run a specific command
lefthook run lint
```

The configuration is in `lefthook.toml` and includes:
- Pre-commit hooks for formatting, linting, and testing
- Pre-push hooks for more comprehensive testing
- Standalone command aliases for common tasks

### Code Style

This project uses hard tabs for indentation. The configuration is in `.rustfmt.toml`.

To format the code according to the project's style guidelines:

```bash
just format
# or
cargo fmt --all
```

### Testing

The project includes comprehensive tests:

```bash
# Run all tests
cargo test

# Run unit tests only
cargo test --lib

# Run integration tests only 
cargo test --test integration_test

# Run CLI tests only
cargo test --test cli_test
```

### Benchmarking

The project includes performance benchmarks:

```bash
# Run all benchmarks 
cargo bench

# Run a specific benchmark
cargo bench -- bench_find_files_small
```

### CI/CD

The repository includes GitHub Actions workflows for:

- Running tests on every push and pull request
- Cross-platform testing (Linux, macOS, Windows)
- Code formatting checks (rustfmt)
- Linting with clippy

## License

MIT
