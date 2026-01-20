# FlashDeal

A high-performance flash sale system built with Rust, designed for backend mastery and systems engineering practice.

## Core Purpose
This is a personal project aimed at mastering:
- **Rust Language**: Ownership, async, traits, and idiomatic patterns.
- **System Design**: Handling high contention, distributed systems, and performance tuning.
- **Backend Concepts**: Pessimistic/Optimistic locking, admission control, idempotency, and observability.

## Architecture
The project follows a hexagonal architecture (Ports & Adapters) to decouple core logic from external dependencies:
- `domain`: Core business entities.
- `logic`: Business rules and workflows.
- `ports`: Interfaces for external systems.
- `adapters`: Concrete implementations (HTTP, DB).
- `app`: Application initialization and runtime.

## Roadmap Overview
The project is divided into several iterative phases:
- **Phase 0: Foundations** - Basic HTTP server and DB integration.
- **Phase 1: Naive FlashDeal** - Transactional purchase flow with pessimistic locking.
- **Phase 2: Observability** - Measuring performance and identifying bottlenecks.
- **Phase 3+: Scaling** - Admission control, async processing, and sharded inventory.

## Getting Started
### Dependencies
- Rust (Latest Stable)
- Docker & Docker Compose
- [k6](https://k6.io/) (for load testing)

### Setup
1. Start the database: `docker-compose up -d postgres`
2. Run migrations: `sqlx migrate run` (from the `server` directory)
3. Start the server: `cargo run`

### Running Load Tests
Since `k6` is running via Docker, use:
```powershell
docker-compose run --rm k6 run /scripts/load_test.js
```
*Note: The k6 container targets `http://host.docker.internal:3000` by default to reach your host machine.*

## Observability
The system is instrumented with Prometheus metrics. You can access the raw metrics at:
- **Metrics Endpoint**: [http://localhost:3000/metrics](http://localhost:3000/metrics)

### Key Metrics
- `http_requests_duration_seconds_bucket`: The histogram buckets for request latency.
- `http_requests_duration_seconds_sum`: Total time spent on requests.
- `http_requests_duration_seconds_count`: Total number of requests.

You can use these to calculate p95/p99 latencies and monitor system throughput.

## Learning Logs
Detailed lessons learned and architectural decisions are recorded in the [Devlogs](docs/devlogs/).
