# Lock to the official Rust 1.95 image
FROM rust:1.95.0-bookworm AS developer

# Install basic development utilities
RUN apt-get update && apt-get install -y \
    git \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Pre-install rust components for optimal VS Code integration
RUN rustup component add rust-analyzer rustfmt clippy
