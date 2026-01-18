# Phase 3: Admission Control Checklists

**Goal:** Protect the database from overload.

## 1. In-Memory Queuing

- [ ] Implement a bounded channel (e.g., `tokio::sync::mpsc`) to buffer requests before hitting the DB
- [ ] Set channel capacity limit (e.g., 100 pending requests)
- [ ] Return `503 Service Unavailable` immediately if channel is full (Fail fast)

## 2. Rate Limiting

- [ ] Add `tower-governor` or `leaky-bucket-lite` middleware
- [ ] Configure rate limit rules (e.g., 10 requests per second per IP)
- [ ] Return `429 Too Many Requests` when limits are exceeded

## 3. Load Testing & Tuning

- [ ] Run load test with concurrency > channel size
- [ ] Verify that the DB connection count remains stable (previously it might have spiked if not limited)
- [ ] Verify that "Queue Overflow" is observed (HTTP 503s appearing)
- [ ] Measure tail latency - it might increase for successful requests due to queue wait time

## 4. Exit Criteria Verification

- [ ] Confirm the database never goes down even under 10x load
