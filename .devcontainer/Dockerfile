FROM rust:1.82.0

# Install necessary tools for cross-compilation
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        gcc-multilib \
        musl-tools \
        libssl-dev \
        pkg-config \
    && rustup target add x86_64-unknown-linux-musl \
    && cargo install --target x86_64-unknown-linux-musl cross

# Set the working directory
WORKDIR /workspace

# Copy the Cargo.toml and Cargo.lock to help with caching dependencies
COPY Cargo.toml Cargo.lock ./

# Create a new empty package
RUN cargo new --bin gh-hook-watcher
WORKDIR /workspace/gh-hook-watcher

# Copy the rest of your application code
COPY . .

# Build the application in release mode for the x86_64 target
RUN cargo build --release --target=x86_64-unknown-linux-musl