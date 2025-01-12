# Stage 1: Build the Rust application
FROM rust:latest AS builder

WORKDIR /usr/src/app

# Copy and fetch dependencies to enable caching
COPY Cargo.toml Cargo.lock ./
COPY src/main.rs ./src/
RUN cargo fetch


# Copy the source code
COPY src ./src

# Build the application
RUN cargo build --release

# Stage 2: Create a minimal runtime image
FROM ubuntu:latest
RUN apt-get update && apt-get -y install ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*
WORKDIR /usr/local/bin


COPY --from=builder /usr/src/app/target/release/srp_backend .

EXPOSE 3000

ENTRYPOINT ["./srp_backend"]
