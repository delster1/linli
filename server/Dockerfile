FROM rust:latest AS builder

WORKDIR /server

# Copy your application source code
COPY . .

# Build the application in release mode
RUN cargo build --release

# Use a smaller image for running the application
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl-dev \
    pkg-config \
    build-essential \
    && rm -rf /var/lib/apt/lists/*


# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/server /app/server

# Expose the port your application listens on
EXPOSE 3333

CMD ["./server"]

