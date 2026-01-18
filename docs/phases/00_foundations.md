# Phase 0: Foundations Checklists

**Goal:** Establish a minimal working backend and make resource limits visible.

## 1. Project Initialization

- [x] Initialize Rust project (`cargo init`)
- [x] Set up project structure (domain, adapters, logic layers)
- [x] Configure `Cargo.toml` dependencies (`axum`, `tokio`, `sqlx`, `serde`, `tower`)

## 2. HTTP Server Setup (Axum)

- [x] Create entry point `main.rs`
- [x] Configure Axum router
- [x] Implement a Health Check endpoint (`GET /health`)
- [x] Implement a basic "Hello World" endpoint to verify routing
- [x] Set up graceful shutdown signal handling

## 3. Database Integration (SQLx + Postgres)

- [x] Spin up Postgres container (Docker Compose)
- [x] Create initial migration (`sqlx migrate add init_schema`)
- [x] Configure `DATABASE_URL` in `.env`
- [x] Implement DB connection pool initialization in `main.rs`
- [x] Verify DB connection on startup

## 4. Configuration & Environment

- [x] Set up environment variable parsing (e.g., `dotenvy`, `config` crate)
- [x] Define helper structs for config (ServerConfig, DatabaseConfig)

## 5. Resource Limits Visibility

- [x] Configure strictly limited connection pool size (e.g., max 5 connections) to make exhaustion easier to trigger
- [x] Log connection pool stats on startup
