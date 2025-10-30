# Stage 1: Build the application
FROM rust:1.90-slim-bullseye AS builder

WORKDIR /api

COPY Cargo.toml ./

COPY src ./src

RUN cargo build --release

# Stage 2: Create the runtime image
FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

RUN useradd -m cake && \
    mkdir -p /app/data && \
    chown cake:cake /app/data

COPY --from=builder /api/target/release/serving-cake /usr/local/bin/serving-cake

USER cake

EXPOSE 8080
ENTRYPOINT ["serving-cake"]
