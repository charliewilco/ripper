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

## License

MIT
