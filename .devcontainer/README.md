# Development Container for Rust

This directory contains configuration for using [Visual Studio Code Remote - Containers](https://code.visualstudio.com/docs/remote/containers) with this project.

## Features

- Rust toolchain with stable channel
- Pre-installed development tools:
  - rustfmt (code formatter)
  - clippy (linter)
  - cargo-watch (file watcher)
  - lefthook (git hooks manager)
  - just (command runner)
  - ripgrep, fd-find, bat (better CLI tools)
- VS Code extensions for Rust development
- Configured for hard tabs (as per project standards)
- Debugging configuration

## Prerequisites

1. [Docker](https://www.docker.com/products/docker-desktop) installed
2. [Visual Studio Code](https://code.visualstudio.com/) installed
3. [Remote - Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers) extension installed in VS Code

## Getting Started

1. Open the project in VS Code
2. Click on the green button in the bottom-left corner (><)
3. Select "Reopen in Container"
4. Wait for the container to build and start (this might take a few minutes the first time)

## Development Commands

Once inside the development container, you can use:

- `just` - View available commands
- `cargo build` - Build the project
- `cargo test` - Run tests
- `cargo watch -x test` - Run tests on file changes
- `cargo clippy` - Run the linter
- `cargo fmt` - Format the code
- `lefthook run pre-commit` - Run pre-commit hooks manually

## VS Code Tasks

Press `Ctrl+Shift+P` and type "Tasks: Run Task" to access:

- **Rust: build project** - Build the project
- **Rust: run tests** - Run all tests
- **Rust: run clippy** - Run the linter
- **Rust: run formatter** - Format code
- **Rust: watch tests** - Continuously run tests on file changes

## Debugging

Press F5 to start debugging with these configurations:

- **Debug executable 'ripper'** - Debug the main binary
- **Debug unit tests** - Debug unit tests
- **Debug integration tests** - Debug integration tests

## Customization

To customize the container:

- Edit `Dockerfile` to install additional tools or packages
- Edit `devcontainer.json` to adjust VS Code settings or extensions
