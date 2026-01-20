# Phase 2: Observability First Checklists

**Goal:** Measure failures before fixing them.

## 1. Structured Logging

- [x] Replace basic `println!` or simple logging with `tracing` and `tracing-subscriber`
- [x] Configure JSON formatting for logs (optional, but good for structured analysis)
- [x] Add Request ID middleware (`tower_http::request_id`)
- [x] Ensure logs include correlation IDs (Trace ID / Request ID)

## 2. Latency Metrics

- [x] Add `metrics` or `prometheus` handler
- [x] Instrument `POST /orders` handler to record execution time (via global metrics middleware)
- [x] Create a histogram metric for "http_requests_duration_seconds"

## 3. Database Observability

- [x] Expose internal SQLx pool metrics (active connections, idle connections, waiters)
- [x] Log warnings if acquiring a DB connection takes longer than X ms
- [ ] (Advanced) Query `pg_stat_activity` to inspect lock waits

## 4. Verification

- [x] Run load test from Phase 1
- [x] Capture P95 and P99 latency while the system is under contention
- [x] Visualise or grep logs to see the "Lock Wait Duration" component of the total latency
- [x] Verified JSON logging and Request ID correlation.
- [x] Verified SQLx pool metrics via Prometheus endpoint.

### Load Test Results (30s, 10 VUs)
- **Throughput**: 88.3 RPS (2,661 total requests)
- **Latency**: avg=11.86ms, median=7.64ms, P95=26.96ms
- **Successful requests** (201): avg=66.13ms, P95=296.86ms (reveals lock contention bottleneck)
- **Checks**: 98.12% passed (expected 409/404 for sold-out scenarios)
