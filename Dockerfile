# ---------- Builder ----------
FROM rust:latest AS builder

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked wasm-bindgen-cli trunk

# Build Frontend
WORKDIR /app/frontend
COPY frontend/ .
RUN trunk build --release

# Build Server
WORKDIR /app/server_rocket
COPY server_rocket/ .
RUN cargo build --release


# ---------- Runtime ----------
FROM debian:bookworm-slim

# Install CA certificates (often needed for outbound HTTPS requests)
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# 1. Recreate the exact directory relationship your code expects
WORKDIR /app
COPY --from=builder /app/frontend/dist /app/frontend/dist

# 2. Change into the server directory so Rocket finds Rocket.toml locally
WORKDIR /app/server_rocket

# 3. Copy ONLY the compiled binary and the config file, leaving the heavy target/ directory behind
COPY --from=builder /app/server_rocket/target/release/server_rocket .
COPY --from=builder /app/server_rocket/Rocket.toml .

EXPOSE 8000

# Force Rocket into release mode
ENV ROCKET_PROFILE=release
ENV ROCKET_ADDRESS=0.0.0.0

# Dynamically map Cloud Run's $PORT to Rocket
CMD ROCKET_PORT=$PORT ./server_rocket
