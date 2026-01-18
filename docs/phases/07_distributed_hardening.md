# Phase 7: Distributed Hardening Checklists

**Goal:** Prepare for multi-node scale.

## 1. External Queue (SQS / Kafka / Redis Streams)

- [ ] Replace in-memory channel with Redis Streams or SQS
- [ ] Implement Consumer Service (can be same binary, different role)
- [ ] Handle message redelivery (Visibility Timeout / Acks)

## 2. Distributed Rate Limiting

- [ ] Replace local `tower-governor` with Redis-backed rate limiter
- [ ] Ensure rate limits apply globally across all instances

## 3. Graceful Degradation

- [ ] Implement Circuit Breakers for Database and External Queue
- [ ] If Queue is down, return fallback response (e.g., "Come back later" or Static Page)
- [ ] Measure impact of partial failures (e.g., Redis down -> Default to local strict limits)

## 4. Exit Criteria Verification

- [ ] Simulate dependency failures (kill Redis) and verify app stays alive
