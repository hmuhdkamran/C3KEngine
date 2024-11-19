#!/bin/bash

# Build all binaries first
cargo build

# Run each binary in the background
cargo run --bin c3k-gate-way &
cargo run --bin c3k-auth-service &
cargo run --bin c3k-hrms-service &
cargo run --bin c3k-retail-service &

# Wait for all background processes to complete
wait