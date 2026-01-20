# FlashDeal Roadmap

**Project:** FlashDeal  
**Audience:** Backend Engineers  
**Intent:** Systems Mastery  
**Version:** 1.0

---

## Phase 0: Foundations

**Status:** Mandatory
[Detailed Checklist](phases/00_foundations.md)

### Objectives

- Establish a minimal working backend
- Make contention and resource limits visible

### Deliverables

- Axum HTTP server
- sqlx Postgres integration
- Ephemeral / local database

### Failure Modes To Observe

- Connection pool exhaustion
- Startup ordering issues

### Exit Criteria

- System boots reliably
- Requests reach the database

---

## Phase 1: Naive FlashDeal

**Status:** Learning
[Detailed Checklist](phases/01_naive_flash_deal.md)

### Objectives

- Implement a correct but slow order flow
- Centralize inventory as a single lock

### Design Choices

- Single inventory row per flash sale
- Pessimistic locking via SELECT FOR UPDATE
- Synchronous transactional HTTP flow

### Failure Modes To Trigger

- Row-level lock contention
- Request queue buildup
- Latency amplification

### Exit Criteria

- System never oversells inventory
- Throughput collapses under load

---

## Phase 2: Observability First

**Status:** Stabilization
[Detailed Checklist](phases/02_observability_first.md)

### Objectives

- Measure failures before fixing them

### Additions

- Structured logging
- Latency metrics
- Database lock inspection

### Signals To Capture

- p95 / p99 request latency
- Active database connections
- Lock wait duration

### Exit Criteria

- All bottlenecks are measurable

---

## Phase 3: Admission Control

**Status:** Control
[Detailed Checklist](phases/03_admission_control.md)

### Objectives

- Protect the database from overload

### Techniques

- In-memory request queue
- Rate limiting

### Failure Modes Eliminated

- Database meltdown

### New Failure Modes

- Queue overflow
- Increased tail latency

### Exit Criteria

- Database remains stable under load

---

## Phase 4: Async Order Processing

**Status:** Scaling
[Detailed Checklist](phases/04_async_order_processing.md)

### Objectives

- Decouple HTTP latency from database writes

### Changes

- Orders are enqueued
- Background workers process orders

### Failure Modes To Observe

- Queue lag
- Worker starvation

### Exit Criteria

- HTTP latency independent of DB speed

---

## Phase 5: Idempotency & Retries

**Status:** Correctness
[Detailed Checklist](phases/05_idempotency_retries.md)

### Objectives

- Ensure retry safety
- Prevent duplicate orders

### Additions

- Idempotency keys
- Retry-safe writes

### Failure Modes Eliminated

- Duplicate orders

### Exit Criteria

- All operations are retry-safe

---

## Phase 6: Inventory Scaling

**Status:** Performance
[Detailed Checklist](phases/06_inventory_scaling.md)

### Objectives

- Reduce lock contention

### Techniques

- Inventory sharding
- Optimistic locking

### Tradeoffs

- Reduced contention
- Increased system complexity

### Exit Criteria

- Throughput increases under load

---

## Phase 7: Distributed Hardening

**Status:** Advanced
[Detailed Checklist](phases/07_distributed_hardening.md)

### Objectives

- Prepare for multi-node scale

### Additions

- External queue (SQS / Kafka)
- Distributed rate limiting
- Graceful degradation

### Failure Modes Accepted

- Eventual consistency
- Partial failure

### Exit Criteria

- System degrades gracefully

---

## Phase 8: Production Readiness

**Status:** Final
[Detailed Checklist](phases/08_production_readiness.md)

### Objectives

- Operational excellence

### Additions

- Health checks
- Autoscaling
- Disaster recovery

### Exit Criteria

- Meets defined SLOs
- Survives chaos testing
