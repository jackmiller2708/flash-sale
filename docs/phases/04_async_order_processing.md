# Phase 4: Async Order Processing Checklists

**Goal:** Decouple HTTP latency from database writes.

## 1. Internal Job Queue

- [ ] Create a struct `OrderJob` (user_id, sale_id)
- [ ] Replace synchronous DB call in `POST /orders` with "Send Job to Queue"
- [ ] Return `202 Accepted` immediately after enqueueing

## 2. Background Worker

- [ ] Spawn a background task (`tokio::spawn`) that consumes from the channel
- [ ] Move the "Transaction & Logic" code from the Controller to the Worker
- [ ] Implement error handling in the worker (Log errors, since we can't return them to HTTP response directly anymore)

## 3. Polling / Status Endpoint (Optional but recommended)

- [ ] Create `GET /orders/status/{job_id}` or similar to check result
- [ ] Or, rely on client polling the order history

## 4. Verification

- [ ] Run load test
- [ ] Observe that HTTP response time is now extremely low (microseconds/milliseconds) regardless of DB load
- [ ] Observe potential "Queue Lag" if the worker is slower than the ingestion rate
