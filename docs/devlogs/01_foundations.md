# Devlog 01: Foundations

## Context
Phase 0 was about setting up the basic infrastructure for a Rust backend. The goal was to have a working Axum server connected to a Postgres database.

## Lessons Learned

### Hexagonal Architecture in Rust
Organizing the project into `domain`, `logic`, `ports`, and `adapters` helps in keeping the code maintainable. Rust's trait system is perfect for defining ports. 
- **Challenge**: Navigating module visibility and crate-level imports.
- **Solution**: Careful use of `pub(crate)` and clear module mappings in `lib.rs`.

### SQLx and Async
SQLx provides compile-time checked queries, which is a game-changer for safety.
- **Insight**: Using `sqlx::query_as!` requires an active database connection or a shared cache (`.sqlx` folder).
- **Tip**: Integrating with `tokio` for the async runtime is seamless with the `tokio-runtime` feature.

### Graceful Shutdown
Implementing a graceful shutdown signal in Axum ensures that the server can finish processing ongoing requests before stopping, which is critical for consistency.

## Next Steps
Moving to real business logic with Phase 1.
