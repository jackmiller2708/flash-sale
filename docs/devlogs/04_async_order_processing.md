# Devlog 04: Async Order Processing

> **Phase 4** | [Detailed Checklist](../phases/04_async_order_processing.md)

**Objectives**: Decouple HTTP response latency from database write latency.

**Additions**: Asynchronous order processing, in-memory status store, status polling endpoint.

---

## The Bottleneck: The Synchronous Wait

After Phase 3, I had a system that was stable but still felt "heavy." I had admission control in place—requests were queued, rate limits enforced, and the database was protected. But there was a catch: **the user was still waiting for the database to finish.**

Every request followed this path:
```
Client → Handler → Queue → [WAIT for Worker + DB] → Response
```

My load tests showed a median latency of **863ms**. While the system was safe from crashing, having a user wait nearly a second for a response is a lifetime in a flash sale. I realized that admission control only solved the *stability* problem, not the *responsiveness* problem.

Phase 4 is about **decoupling**: accepting the request immediately and dealing with the database later.

---

## Design Decision: The Async Response Pattern

The logic is simple: if I've successfully put the request in the queue, I've essentially "accepted" the responsibility of processing it. Why make the client wait for the actual COMMIT?

I decided on a `202 Accepted` pattern:
1. Client POSTs an order.
2. Server validates the request and enqueues it.
3. Server returns `202 Accepted` with a URL to check the status.
4. Client polls the status URL until the order is `completed` or `failed`.

## Design Decision: High-Concurrency State Management

Initially, I used a global `Arc<RwLock<HashMap<...>>>` to track order status. It was simple, but it quickly became a bottleneck. 

### The "Thundering Herd" Problem

When I switched to `try_send()` for the order queue (to fail fast with a 503 instead of blocking), my performance metrics actually got *worse*. 

The reason? **Lock Contention.** 

Every single request was competing for a **Write Lock** on the global `HashMap`. Even if a request was going to fail because the queue was full, it still had to wait in line for that global lock just to insert the "Pending" status and then wait again to remove it. You’ve effectively turned a multi-threaded server into a single-threaded one.

### The Solution: Sharded State with DashMap

I migrated the status store to `dashmap`. Unlike a standard `RwLock`, `DashMap` uses **sharding**. It splits the map into many smaller buckets, each with its own lock. This allows multiple threads to insert, remove, and read simultaneously without blocking the entire system.

```rust
// From global lock...
pub order_status_store: Arc<RwLock<HashMap<Uuid, OrderProcessingStatus>>>,

// ...To sharded locks
pub order_status_store: Arc<dashmap::DashMap<Uuid, OrderProcessingStatus>>,
```

This architectural shift allows the server to handle high request volume without the "Thundering Herd" effect.

---

## Refinement: The Worker as a "Deferred Handler"

This decision led to a subtle but important architectural shift. In Phase 0-2, my HTTP handlers were the "adapters" that managed database transactions. In Phase 3, the worker just executed logic.

In Phase 4, I realized the **Background Worker is also an Adapter**.

Just like a product handler in `product_handler.rs` manages the lifecycle of a request, the worker manages the lifecycle of a queued job. This means the worker should be responsible for its own transaction management, keeping the logic in `order_logic.rs` pure and unaware of whether it's being called from an HTTP thread or a background task.

---

## Implementation: In-Memory Status Tracking

To make this work, I needed a way to track the "in-flight" state of an order. I implemented an in-memory `order_status_store`:

```rust
pub enum OrderProcessingStatus {
    Pending,
    Completed(Order),
    Failed(String),
}
```

### Upfront ID Generation

The biggest "Aha!" moment was realizing I couldn't wait for the DB to generate the order ID. I had to generate a `UUIDv4` **before** enqueuing the request. This allowed me to return the ID to the client immediately, which they then use to poll the status.

```rust
// 1. Try to reserve a slot in the queue first (The "Reserve" Pattern)
let permit = state.order_queue_tx.try_reserve().map_err(|_| ApiError::service_unavailable(...))?;

// 2. Generate ID upfront if we have a guaranteed slot
let order_id = Uuid::new_v4();

// 3. Mark as pending in status store (DashMap is synchronous)
state.order_status_store.insert(order_id, OrderProcessingStatus::Pending);

// 4. Send using the permit
permit.send(OrderQueueMessage { order_id, command });
```

### The Transactional Worker

I updated the worker to act as that "deferred handler." It starts the transaction, calls the core logic, and commits—all while updating the in-memory status store so the client sees the progress.

```rust
while let Some(msg) = rx.recv().await {
    let OrderQueueMessage { order_id, command } = msg;

    // The worker acts as a deferred handler, managing the transaction lifecycle
    let mut tx = match db_pool.begin().await {
        Ok(conn) => conn,
        Err(e) => {
            order_status_store.insert(
                order_id,
                OrderProcessingStatus::Failed(format!("DB connection failed: {}", e)),
            );
            continue;
        }
    };

    let result = create_order(&mut *tx, ...).await;
    let commit_result = tx.commit().await;

    if commit_result.is_err() {
        order_status_store.insert(
            order_id,
            OrderProcessingStatus::Failed(format!("Transaction commit failed: {}", commit_result.unwrap_err())),
        );
        continue;
    }

    // Store result in status store (synchronous insert)
    let status = match result {
        Ok(order) => OrderProcessingStatus::Completed(order),
        Err(e) => OrderProcessingStatus::Failed(e.to_string()),
    };

    order_status_store.insert(order_id, status);
}
```

---

## Performance: Measuring the Real Impact

When I first ran the new load tests, I saw a raw "ingestion" latency of **8ms**. That's the time it takes to validate the request and hit the queue. It felt like a massive win—a 99% reduction!

### The Polling Reality Check

However, a real user's experience isn't just the `POST` request; it's the `POST` plus the time spent polling. I updated my `k6` script to simulate a realistic client polling every 1 second.

**Baseline (Phase 3 Sync)** vs **Final (Phase 4 Optimized + Release Mode)**:

| Metric              | Phase 3 (Sync) | Phase 4 (Final Release) | Improvement |
| :------------------ | :------------- | :---------------------- | :---------- |
| **Median Duration** | 863ms          | **5.23ms**              | **99.4% ↓** |
| **P95 Duration**    | 1.2s           | **25.11ms**             | **97.9% ↓** |
| **Throughput**      | 163 req/s      | **352.2 req/s**         | **116% ↑**  |

The median duration is now effectively the cost of a network round-trip. Even with 150 concurrent users and a serial worker, the ingestion path is so fast that the "waiting room" (the queue) never feels like a bottleneck to the HTTP response.

---

## Implementation Challenges

### 1. The Polling Frequency Trap

Initially, my test script was polling every 100ms and immediately retrying on failures. This created a tight loop where 150 VUs were hammered the server as fast as possible. I learned to chill out the client—switching to a **1-second polling interval** and adding a **100ms pause on failures** provided a good user experience without overwhelming the system.

### 2. State Management vs. Deferred Processing

#### Bottleneck Migration

The goal of Phase 4 was to "defer" the heavy database work to a background worker. However, to track that work, the HTTP handler still needs to perform some "immediate" state management (inserting the `Pending` status). 

If that immediate management uses a naive global lock (like my initial `RwLock<HashMap>`), then **the handler is still synchronous**. It’s just waiting for a lock instead of waiting for a database. Your system is only as fast as its slowest synchronous component; if every request has to wait 200ms for a global status lock, it doesn't matter if the database work is deferred—the user still feels the lag.

**Decoupling only works if your tracking mechanism is several orders of magnitude faster than the work you are deferring.** `DashMap` and the ["Reserve" pattern](#3-optimization-the-reserve-pattern) ensured that my "immediate" work stayed in the sub-millisecond range.

#### Full Queue Bug

Then I hit a bug where a full queue would return an error, but the `order_status_store` already had a `Pending` entry. The order would stay "Pending" forever in the user's UI. I had to ensure that if enqueuing fails, I immediately clean up the status store:

```rust
// Optimized: Try to reserve a slot before doing any state management
let permit = state.order_queue_tx.try_reserve().map_err(|_| {
    metrics::counter!("order_queue_overflow_total").increment(1);
    ApiError::service_unavailable(...)
})?;

state.order_status_store.insert(order_id, OrderProcessingStatus::Pending);
permit.send(queue_msg);
```

---

## The Performance Trap: Median Latency in Local Benchmarks

When I first ran the optimized `DashMap` version, I was shocked to see a median latency of **242ms** for 503 responses. In theory, a failed `try_send` should take microseconds. This led me to three critical realizations about high-performance benchmarking in Rust.

### 1. Debug Mode is the Processor Killer

If you run with `cargo run`, you are in **Debug mode**. In Rust, the difference between Debug and Release performance is often 10x to 100x. Debug mode includes no optimizations and adds heavy runtime checks. Under the pressure of 150 VUs, the overhead of the async runtime and shared state in Debug mode is enough to peg the CPU and drive latency into the hundreds of milliseconds.

> [!IMPORTANT]
> Always use `cargo run --release` for performance measurements.

### 2. Logging is Synchronous I/O

I had my logging set to `DEBUG`. At 200+ requests per second, the server was generating massive amounts of JSON logs and writing them to a Windows terminal. Terminals are notoriously slow at consuming output. If the terminal can't keep up, the `tracing` subscriber eventually blocks the thread to wait for I/O, killing your response times.

I switched the default log level to `INFO` to keep the ingestion path clean.

### 3. Optimization: The "Reserve" Pattern

Initially, I was performing an `insert` and then a `remove` on the `DashMap` for every rejected request. While `DashMap` is fast, doing two atomic-heavy operations for a request you're about to throw away is wasteful.

I refactored the handler to use `try_reserve()` on the channel FIRST. This ensures we only touch the status store if we are **guaranteed** a slot in the queue:

```rust
// 1. Check queue capacity first (No lock touched yet)
let permit = state.order_queue_tx.try_reserve()?;

// 2. Only now generate ID and track status
state.order_status_store.insert(order_id, ...);

// 3. Send using the guaranteed permit
permit.send(msg);
```

This "fail-fast" path is now practically free, allowing the server to reject thousands of requests per second without breaking a sweat.

---

## Phase 4 Complete: The Concurrency Grand Slam

The exit criteria for Phase 4 were ambitious, and the final results exceeded them:
1. ✅ **99.4% reduction in ingestion latency** - 5.23ms median achieved.
2. ✅ **116% increase in throughput** - 352 req/s handled on a single core system.
3. ✅ **High-concurrency state management** - `DashMap` eliminated all global lock contention.
4. ✅ **Zero-Churn 503s** - `try_reserve` ensures we don't waste work on rejected requests.

Phase 4 transformed a slow, synchronous API into a high-performance ingestion engine. We've mastered the art of "Deferred Processing" and sharded state management.

## What's Next

Phase 4 proved that async processing is the way forward for responsiveness. But I'm still using an in-memory map for state, and I haven't even touched idempotency yet. If a user double-clicks the buy button, they'll get two order entries.

Phase 5 will be about **Idempotency & Retries**:
1. **Idempotency Keys**: Ensuring one user, one order, no matter how many times they click.
2. **Database-backed Status**: Moving the status store into Postgres for durability.
3. **Retry-safe Writes**: Ensuring the system can recover gracefully from partial failures.

The system is getting fast. Now let's make it correct and durable.
