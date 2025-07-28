# --- Stage 1: Build Rust binary ---
FROM rust:1.88 as builder

WORKDIR /build
COPY . .
RUN cargo build --release

# --- Stage 2: Runtime image ---
FROM debian:bookworm-slim

# Install necessary libraries (for HTTPS, etc.)
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy compiled Rust binary from builder stage
COPY --from=builder /build/target/release/backend .

# Optional: Copy entire source tree to /src (for VS Code, debugging, or cargo-watch dev in container)
COPY --from=builder /build /src

EXPOSE 3000

CMD ["./backend"]

