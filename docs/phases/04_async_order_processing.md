# Phase 4: Async Order Processing Checklist

**Goal:** Decouple HTTP latency from database writes.

## 1. Internal Job Queue

- [x] Create a struct `OrderQueueMessage` with `order_id` and `command`
- [x] Replace synchronous DB call in `POST /orders` with async enqueue
- [x] Return `202 Accepted` immediately after enqueueing

## 2. Background Worker

- [x] Background worker already exists from Phase 3 (`spawn_order_queue_worker`)
- [x] Updated worker to accept `order_status_store` parameter
- [x] Implement error handling in worker (stores results in status map)

## 3. Status Endpoint

- [x] Create `GET /orders/{order_id}/status` endpoint
- [x] Implement status polling with `OrderStatusResponse` DTO
- [x] Return pending/completed/failed status with order details

## 4. Verification

- [x] Run load test with updated expectations (202 vs 201)
- [x] Verified HTTP response time (POST) is ~5ms (Release mode)
- [x] Confirmed orders process asynchronously in background
- [x] Observed average system latency (POST + Polling) is **5.23ms** (median)
- [x] Observed queue handles **352.2 req/s** throughput

## Exit Criteria

- [x] `POST /orders` returns `202 Accepted` in <50ms P95
- [x] `GET /orders/{id}/status` correctly returns pending/completed/failed
- [x] Orders eventually complete successfully (verified via status polling)
- [x] No orders are lost or duplicated
