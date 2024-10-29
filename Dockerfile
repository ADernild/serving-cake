# Stage 1: Build the application
FROM rust:1.79 AS builder

# Set the working directory
WORKDIR /api

# Copy over your manifests
COPY Cargo.toml Cargo.lock ./

# Copy over your source files
COPY src ./src

# Build the project
RUN cargo build --release

# Stage 2: Create the runtime image
FROM ubuntu:22.04

# Install necessary dependencies for Diesel
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /api/target/release/serving-cake /usr/local/bin/serving-cake

EXPOSE 8080

# Set the entrypoint to the compiled binary
ENTRYPOINT ["serving-cake"]
