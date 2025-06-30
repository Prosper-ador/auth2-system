# ---------- Stage 1: Build ----------
FROM rust:1.78-slim AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
COPY rust-toolchain.toml ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -r src

# Copy actual source code
COPY src ./src

# Build optimized release binary
RUN cargo build --release

# Strip debug symbols to reduce binary size
RUN strip target/release/auth_api

# ---------- Stage 2: Runtime ----------
FROM ubuntu:22.04

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/auth_api ./auth_api

# Make binary executable
RUN chmod +x ./auth_api

# Expose port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=60s --retries=3 \
  CMD curl -f http://localhost:3000/health || exit 1

# Run the application
CMD ["./auth_api"]
