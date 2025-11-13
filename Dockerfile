# Build stage
FROM rust:1.83-slim AS builder

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy manifests
COPY Cargo.toml ./

# Copy source code
COPY src ./src

# Build for release
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install ca-certificates for HTTPS
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/target/release/six7 /usr/local/bin/six7

# Copy default config
COPY six7.yaml /app/six7.yaml

# Create data directory
RUN mkdir -p /data

# Expose port
EXPOSE 4040

# Set environment
ENV RUST_LOG=info

# Run the binary
CMD ["six7"]
