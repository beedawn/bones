# Stage 1: Build the application
FROM rust:1.75 AS builder

# Create a new empty shell project
RUN USER=root cargo new --bin actix-app
WORKDIR /actix-app

# Copy the Cargo.toml and Cargo.lock files
COPY ./bones/Cargo.toml ./bones/Cargo.lock ./

# Copy the source code
COPY ./bones/src ./src


# Copy the .env file
COPY ./.env ./

# Build the application
RUN cargo build --release

# Stage 2: Create a smaller image for the runtime
FROM debian:bookworm-slim

# Install OpenSSL (or any other libraries your app needs)
RUN apt-get update && apt-get install -y \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /actix-app/target/release/bones /usr/local/bin/bones

# Expose the port that the Actix app will run on
EXPOSE 8080

# Run the Actix app
CMD ["bones"]

