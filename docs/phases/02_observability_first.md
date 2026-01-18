# Phase 2: Observability First Checklists

**Goal:** Measure failures before fixing them.

## 1. Structured Logging

- [ ] Replace basic `println!` or simple logging with `tracing` and `tracing-subscriber`
- [ ] Configure JSON formatting for logs (optional, but good for structured analysis)
- [ ] Add Request ID middleware (`tower_http::request_id`)
- [ ] Ensure logs include correlation IDs (Trace ID / Request ID)

## 2. Latency Metrics

- [ ] Add `metrics` or `prometheus` handler
- [ ] Instrument `POST /purchase` handler to record execution time
- [ ] Create a histogram metric for "purchase_request_duration_seconds"

## 3. Database Observability

- [ ] Expose internal SQLx pool metrics (active connections, idle connections, waiters)
- [ ] Log warnings if acquiring a DB connection takes longer than X ms
- [ ] (Advanced) Query `pg_stat_activity` to inspect lock waits

## 4. Verification

- [ ] Run load test from Phase 1
- [ ] Capture P95 and P99 latency while the system is under contention
- [ ] Visualise or grep logs to see the "Lock Wait Duration" component of the total latency
