# Phase 1: Naive FlashDeal Checklists

**Goal:** Implement a correct but slow order flow using pessimistic locking.

## 1. Domain Modeling

- [x] Define `FlashSale` struct/entity (id, total_inventory, start_time, end_time)
- [x] Define `Order` struct/entity (id, user_id, flash_sale_id, status)
- [x] Create SQL migrations for `flash_sales` and `orders` tables

## 2. Inventory Management (The "Naive" Approach)

- [x] Seed initial data: 1 Flash Sale, 100 items inventory
- [x] Implement Repository method to fetch Flash Sale with lock (`SELECT * FROM flash_sales WHERE id = $1 FOR UPDATE`)
- [x] Implement Repository method to decrement inventory

## 3. Order Transaction Flow

- [x] Create `POST /orders` handler
- [x] Start DB transaction
- [x] Fetch flash sale (locking the row)
- [x] Check if inventory > 0
- [x] If yes: Decrement inventory, Insert Order, Commit transaction
- [x] If no: Rollback/Return error "Sold Out"
- [x] Return success/failure response

## 4. Testing Failure Modes

- [x] Create a load test script (e.g., `k6` or simple bash loop) to spam the order endpoint
- [x] Verify that no inventory is oversold (Correctness)
- [x] Observe latency increase as requests queue up for the lock (Failure Mode: Latency Amplification)
