# --- Stage 1: Build ---
    FROM rust:1.84-slim-bookworm AS builder
    # Install build dependencies (needed for some crates)
    RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
    
    WORKDIR /app
    
    # Step 1: Cache dependencies
    # Creating a dummy project to build dependencies first
    COPY Cargo.toml Cargo.lock ./
    RUN mkdir src && echo "fn main() {}" > src/main.rs
    RUN cargo build --release
    RUN rm -f src/main.rs target/release/deps/url_shortener*
    
    # Step 2: Build the actual source
    COPY . .
    RUN cargo build --release
    
    # --- Stage 2: Runtime ---
    # Using Google's Distroless for the smallest, most secure footprint
    FROM gcr.io/distroless/cc-debian12
    
    WORKDIR /app
    
    # Copy the binary from the builder stage
    COPY --from=builder /app/target/release/url_shortener /app/url_shortener
    
    # Expose the port your Axum app uses
    EXPOSE 8000
    
    # Run it!
    CMD ["./url_shortener"]