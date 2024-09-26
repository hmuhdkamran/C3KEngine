#!/bin/bash

cargo run --bin c3k-gate-way &
cargo run --bin c3k-auth-service &
cargo run --bin c3k-hrms-service &
cargo run --bin c3k-retail-service &
wait