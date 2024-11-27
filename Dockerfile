# Stage 1: Build frontend & server
FROM rust:latest as builder

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk wasm-bindgen-cli

WORKDIR /app/frontend
COPY frontend/Cargo.toml .
COPY frontend/index.html .
COPY frontend/index.css .
COPY frontend/Images/ ./Images/
COPY frontend/MP3/ ./MP3/
COPY frontend/src/ ./src/

RUN cargo install trunk
RUN trunk build --release

WORKDIR /app/server_rocket
COPY server_rocket/Cargo.toml .
COPY server_rocket/Rocket.toml .
COPY server_rocket/src/ ./src/

RUN cargo build --release

# Stage 2: Create the final image
FROM rust:latest

WORKDIR /app
COPY --from=builder /app/frontend/dist /app/frontend/dist
COPY --from=builder /app/server_rocket /app/server_rocket

EXPOSE 8000
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

WORKDIR /app/server_rocket
CMD ["./target/release/server_rocket"]
