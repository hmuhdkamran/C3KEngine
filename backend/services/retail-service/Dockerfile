# Use the official Rust image as the base image
FROM rust:1.84.1 as builder

# Set the working directory in the container
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Download and cache dependencies
RUN cargo build --release

# Copy the source code
COPY . .

# Build the application
RUN cargo build --release

# Use a minimal base image to run the application
FROM debian:buster-slim

# Install necessary dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/app/target/release/retail_service /app/retail_service

# Expose the port the app runs on
EXPOSE 8203

# Health check endpoint
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8203/api/retail/health_check || exit 1

# Run the application
CMD ["./retail_service"]