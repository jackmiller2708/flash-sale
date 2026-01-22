# FlashDeal

A high-performance flash sale system built with Rust, designed for backend mastery and systems engineering practice.

## Core Purpose
This is my personal project where I'm mastering:
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
I'm building this project through several iterative phases:
- **Phase 0: Foundations** ✅ - Basic HTTP server and DB integration | [Devlog](docs/devlogs/00_foundations.md)
- **Phase 1: Naive FlashDeal** ✅ - Transactional order flow with pessimistic locking | [Devlog](docs/devlogs/01_naive_flash_deal.md)
- **Phase 2: Observability First** ✅ - Measuring performance and identifying bottlenecks | [Devlog](docs/devlogs/02_observability_first.md)
- **Phase 3+: Scaling** 🚧 - Admission control, async processing, and sharded inventory

## Getting Started
### Dependencies
- Rust (Latest Stable)
- Docker & Docker Compose
- [k6](https://k6.io/) (for load testing)

### Setup
1. Start the core database:
   ```powershell
   docker-compose up -d postgres
   ```
2. Run migrations: `sqlx migrate run` (from the `server` directory)
3. Start the server: `cargo run`

## Modular Service Control
The project uses Docker Compose **profiles** to let you start exactly what you need.

| Command                                                                | Purpose                                      |
| :--------------------------------------------------------------------- | :------------------------------------------- |
| `docker-compose up -d postgres`                                        | Start only the database (default).           |
| `docker-compose --profile observability up -d`                         | Start Prometheus and Grafana for monitoring. |
| `docker-compose --profile tools run --rm k6 run /scripts/load_test.js` | Run a load test.                             |
| `docker-compose --profile "*" up -d`                                   | Start everything.                            |

## Observability
Once the `observability` profile is running:
- **Prometheus**: [http://localhost:9090](http://localhost:9090) (Scrapes the Rust server).
- **Grafana**: [http://localhost:3001](http://localhost:3001) (Pre-configured with anonymous admin access).
- **Metrics Endpoint**: [http://localhost:3000/metrics](http://localhost:3000/metrics) (Raw data from server).

### Key Metrics
- `http_requests_duration_seconds_bucket`: The histogram buckets for request latency.
- `http_requests_duration_seconds_sum`: Total time spent on requests.
- `http_requests_duration_seconds_count`: Total number of requests.

You can use these to calculate p95/p99 latencies and monitor system throughput.

## Development Journal

I'm documenting my journey building this system in the [Devlogs](docs/devlogs/). Each devlog corresponds to a phase of development and includes:

- **Technical decisions and trade-offs**: Why I chose pessimistic locking, how I structured repositories, etc.
- **Challenges and solutions**: Real problems I ran into (Docker networking, foreign key violations, module visibility)
- **Performance insights**: Load testing results, metrics analysis, and bottleneck identification
- **Lessons learned**: What worked, what didn't, and what I'd do differently

The devlogs are written in a conversational style—think of them as notes I'm sharing with a colleague, not formal documentation. Each devlog references its corresponding [phase checklist](docs/phases/) for detailed implementation steps. If you're learning Rust, building high-concurrency systems, or just curious about the thought process behind technical decisions, they might be useful.
