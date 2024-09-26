@echo off

:: Build all binaries first
cargo build

:: Run each binary in parallel (start each process in a new window)
start /b target\debug\c3k-gate-way
start /b target\debug\c3k-auth-service
start /b target\debug\c3k-hrms-service
start /b target\debug\c3k-retail-service

:: Wait for all processes to complete
:: (Batch scripts don't have an explicit "wait" for background processes.
::  If necessary, you can manually check for process completion.)