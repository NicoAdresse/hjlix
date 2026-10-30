# Dockerfile

# AI generated using Gemini 3.5

# Stage 1: Build environment
FROM rust:1.97.1-bookworm AS builder

WORKDIR /app

# Copy dependency manifests
COPY Cargo.toml Cargo.lock ./

# Create dummy src to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy actual source code
COPY src/ ./src/

# Touch main.rs to ensure cargo rebuilds with actual code
RUN touch src/main.rs
RUN cargo build --release

# Stage 2: Runtime environment (Ubuntu)
FROM ubuntu:24.04

WORKDIR /app

# Install necessary runtime certificates or utilities if needed
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/hjlix /usr/local/bin/hjlix

# Set the default entrypoint
ENTRYPOINT ["hjlix"]
