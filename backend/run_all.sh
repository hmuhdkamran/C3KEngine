#!/bin/bash

# Build all binaries first
cargo build

# Run each binary in the background
./target/debug/c3k-gate-way &
./target/debug/c3k-auth-service &
./target/debug/c3k-hrms-service &
./target/debug/c3k-retail-service &

# Wait for all background processes to complete
wait