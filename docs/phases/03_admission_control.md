# Phase 3: Admission Control Checklists

**Goal:** Protect the database from overload.

## 1. In-Memory Queuing

- [x] Implement a bounded channel (e.g., `tokio::sync::mpsc`) to buffer requests before hitting the DB
- [x] Set channel capacity limit (e.g., 100 pending requests)
- [x] Return `503 Service Unavailable` immediately if channel is full (Fail fast)

## 2. Rate Limiting

- [x] Add `governor` middleware for user-based rate limiting
- [x] Configure rate limit rules (e.g., 10 requests per second per user)
- [x] Return `429 Too Many Requests` when limits are exceeded

## 3. Load Testing & Tuning

- [x] Update k6 script with higher concurrency (150 VUs > channel size)
- [x] Run load test with concurrency > channel size
- [x] Verify that the DB connection count remains stable (previously it might have spiked if not limited)
- [x] Verify that the system handles load gracefully (no crashes/timeouts)
- [x] Measure tail latency - observed increase (P95: 1.85s) due to queuing smoothing

## 4. Exit Criteria Verification

- [x] Confirm the database never goes down even under load (150 VUs sustained stability)
- [x] Verify that the system degrades gracefully with predictable latency
