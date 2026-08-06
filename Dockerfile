# Developer stage
# Add basic tools in the image
# Lock to the official Rust 1.95 image
FROM rust:1.95.0-bookworm AS developer

# Install basic development utilities
RUN apt-get update && apt-get install -y \
    git \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Pre-install rust components for optimal VS Code integration
RUN rustup component add rust-analyzer rustfmt clippy

# Build stage
# Compile application
FROM rust:1.95.0-bookworm AS build

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

# Make a dummy build to compile dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy code for MTG tracker
COPY . .

RUN cargo build --release

FROM rust:1.95.0-slim-bookworm AS runtime

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=build /app/target/release/app /usr/local/bin/app

ENTRYPOINT ["app"]