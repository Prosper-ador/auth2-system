# Multi-stage build for Rust application
FROM rust:1.78-alpine as builder


# Install build dependencies
RUN apk add --no-cache musl-dev

# Set working directory
WORKDIR /app

# Copy Cargo files
COPY Cargo.toml Cargo.lock* ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (this layer will be cached)
RUN cargo build --release

# Remove dummy main.rs and copy actual source code
RUN rm src/main.rs
COPY src ./src

# Build the application
RUN cargo build --release

# Runtime stage
FROM alpine:latest

# Install runtime dependencies
RUN apk add --no-cache ca-certificates

# Create non-root user
RUN addgroup -g 1001 -S appgroup && \
    adduser -u 1001 -S appuser -G appgroup

# Set working directory
WORKDIR /app

# Copy binary from builder stage
COPY --from=builder /app/target/release/auth_api /app/auth_api

# Change ownership to non-root user
RUN chown appuser:appgroup /app/auth_api

# Switch to non-root user
USER appuser

# Expose port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:3000/health || exit 1

# Run the application
CMD ["./auth_api"]
