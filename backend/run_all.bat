@echo off
start cargo run --bin c3k-gate-way
start cargo run --bin c3k-auth-service
start cargo run --bin c3k-hrms-service

:: Wait for all processes to finish
pause