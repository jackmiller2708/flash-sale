# Phase 5: Idempotency & Retries Checklists

**Goal:** Ensure retry safety and prevent duplicate orders.

## 1. Idempotency Key Schema

- [ ] Add `idempotency_key` column to `orders` table (unique constraint)
- [ ] Update `POST /orders` to require `Idempotency-Key` header

## 2. Logic Implementation

- [ ] Modify Order logic:
  - Check if Order with `idempotency_key` already exists
  - If yes, return the _existing_ order status/result (Success)
  - If no, proceed with inventory check and creation
- [ ] Handle race conditions (Concurrent inserts with same key) -> Catch Unique Constraint Violation and return matched result

## 3. Client Retry Logic (Simulation)

- [ ] Update load test script to reuse Idempotency Keys for a percentage of requests
- [ ] Verify that no duplicate orders are created
- [ ] Verify that retried requests get the correct success response

## 4. Exit Criteria Verification

- [ ] Verify system handles network blips (client retries) gracefully without double-charging
