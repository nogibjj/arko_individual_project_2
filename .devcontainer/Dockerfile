# Use a lightweight Debian-based image as a base for Rust
FROM debian:bullseye-slim

# Install Rust and build dependencies
RUN apt-get update && \
    apt-get install -y curl build-essential && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    . "$HOME/.cargo/env" && \
    rustup default stable && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /workspace

# Copy all files to the container
COPY . .

# Build the Rust project
RUN . "$HOME/.cargo/env" && cargo build --release

# Set the command to execute the binary
CMD ["./target/release/rust_etl_cli"]
