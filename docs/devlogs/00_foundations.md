# Devlog 00: Foundations

> **Phase 0** | [Detailed Checklist](../phases/00_foundations.md)

**Objectives**: Establish a minimal working backend and make resource limits visible.

---

## Why I'm Building This

I've been wanting to level up my backend skills for a while now, and Rust kept coming up in conversations about high-performance systems. Flash sales seemed like the perfect playground—they're conceptually simple (sell limited items fast) but deceptively complex when you dig into the concurrency and consistency challenges. Plus, I wanted to build something I could actually measure and optimize, not just another CRUD app.

So here we are: a flash sale system built from scratch in Rust. The goal isn't just to make it work—it's to understand *why* it works, where it breaks, and how to make it better.

## Setting Up the Foundation

Phase 0 was all about getting the basics right: a working Axum server talking to a Postgres database. Sounds straightforward, but there were definitely some learning curves.

### Hexagonal Architecture in Rust

I organized the project into layers: `domain` for business entities, `logic` for workflows, `ports` for interfaces, and `adapters` for concrete implementations (HTTP, database, etc.). Rust's trait system is *perfect* for this pattern—traits naturally define the boundaries between layers.

The tricky part? Module visibility. Coming from other languages, Rust's privacy rules felt restrictive at first. I spent way too much time fighting the compiler about what should be `pub`, `pub(crate)`, or private. Eventually, I learned to lean into it—the compiler was forcing me to think about my API surface area, which is actually a good thing.

### SQLx and the Magic of Compile-Time Checking

SQLx is honestly one of the coolest libraries I've used. The `query_as!` macro checks your SQL queries *at compile time* against your actual database schema. The first time I saw it catch a typo in a column name before I even ran the code, I was hooked.

There's a catch though: it needs either an active database connection during compilation or a cached metadata file (`.sqlx` folder). I went with the cache approach for CI/CD friendliness. Running `cargo sqlx prepare` after schema changes became part of my workflow.

Integrating with Tokio's async runtime was surprisingly smooth. The `tokio-runtime` feature just works, and suddenly I had async database queries without much ceremony.

### Docker Compose for Development

I set up Docker Compose to manage the Postgres database and eventually other services. Early on, I introduced **profiles** to keep things modular:

- **Default profile**: Just Postgres (the bare minimum to run the server)
- **Observability profile**: Prometheus and Grafana for monitoring (added in Phase 2)
- **Tools profile**: k6 for load testing (also Phase 2)

This meant I could run `docker-compose up -d` and get just the database, or `docker-compose --profile observability up -d` when I needed the full monitoring stack. It kept my local environment clean and fast.

### Making Resource Limits Visible

One of Phase 0's objectives was to make resource limits visible early. I configured the connection pool with a deliberately small size (5 connections) to make exhaustion easier to trigger during testing. This might seem counterintuitive, but it's better to hit limits in development than in production.

I also added logging for connection pool stats on startup. This way, I could see at a glance how many connections were active, idle, or waiting. It's the kind of visibility that becomes critical when debugging performance issues later.

### Graceful Shutdown

One thing I made sure to implement early was graceful shutdown. When you hit Ctrl+C, the server should finish processing in-flight requests before shutting down. Axum makes this easy with signal handling, and it's critical for maintaining data consistency—you don't want to drop a transaction halfway through because someone restarted the server.

## What I Learned

**Rust's ownership model is strict, but it pays off.** The compiler caught so many potential bugs that would've been runtime errors in other languages. Yes, it slowed me down at first, but I'm already seeing the benefits.

**Async Rust has a learning curve.** Understanding `async`/`await`, `Future`, and when to use `tokio::spawn` vs just `.await` took some time. The mental model is different from synchronous code, but once it clicks, it's powerful.

**Tooling matters.** SQLx's compile-time checks, `cargo-watch` for auto-reloading, and Docker Compose for environment management made development so much smoother. Investing time in good tooling early on pays dividends.

## Phase 0 Complete

The exit criteria for Phase 0 were simple: the system boots reliably, and requests reach the database. ✅ Done.

More importantly, I had visibility into resource limits (connection pool stats) and a solid foundation to build on. The architecture was clean, the tooling was in place, and I understood the basics of async Rust and SQLx.

## What's Next

The foundation is solid. Now comes the fun part: implementing the actual flash sale logic. Phase 1 is all about getting the core order flow working—even if it's naive and doesn't scale yet. I'll worry about performance later. First, I need to make it *correct*.
