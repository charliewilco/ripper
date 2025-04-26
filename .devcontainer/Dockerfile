FROM mcr.microsoft.com/devcontainers/rust:latest

# Install additional CLI tools
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
    ripgrep \
    fd-find \
    bat \
    htop \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Set up Rust toolchain
RUN rustup update stable && \
    rustup component add rustfmt clippy && \
    rustup target add wasm32-unknown-unknown && \
    cargo install cargo-edit cargo-watch cargo-expand lefthook just

# Set up git config for hard tabs
RUN git config --global core.editor "code --wait" && \
    git config --global core.autocrlf input && \
    git config --global init.defaultBranch main

# Create bash aliases
RUN echo 'alias ls="ls --color=auto"' >> /home/vscode/.bashrc && \
    echo 'alias ll="ls -la"' >> /home/vscode/.bashrc && \
    echo 'alias grep="grep --color=auto"' >> /home/vscode/.bashrc

# Set up a default welcome message
RUN echo 'echo "🦀 Rust development environment ready! 🦀"' >> /home/vscode/.bashrc && \
    echo 'echo "- Use \"just\" command to see available tasks"' >> /home/vscode/.bashrc && \
    echo 'echo "- Use \"cargo watch -x test\" for continuous testing"' >> /home/vscode/.bashrc

# Set ownership
USER vscode
