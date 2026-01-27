# Devlog 05: Idempotency & Retries

> **Phase 5** | [Detailed Checklist](../phases/05_idempotency_retries.md)

**Objectives**: Ensure exactly-once semantics for order creation and handle network retries gracefully.

**Additions**: Idempotency keys, strict header validation, deterministic UUID v5 generation, race condition handling.

---

## The Problem: Network Failures and Double-Clicks

After Phase 4, the system was responsive and fast. I had async processing with 5.23ms median latency and 352 req/s throughput. But there was a fundamental correctness problem: **every request created a new order**, even if it was a retry.

In distributed systems, network failures are the norm. When a client sends a request and the response is lost, they have no way to know if the server processed it. If they retry, one of two things happens:
1. The first request never made it (safe to retry).
2. The first request succeeded, but the response was lost (dangerous to retry).

Without idempotency, scenario #2 results in duplicate orders. In a flash sale, this means overselling inventory and angry customers.

---

## Design Decision: Server-Side Validation is Non-Negotiable

My first instinct was to be "backward compatible" by generating a UUID on the server if the client forgot to send one. This seemed nice and developer-friendly.

But this creates a silent failure mode. If a client *intends* to be idempotent but sends `"undefined"` (a common JavaScript bug), the server would happily generate a fresh UUID for every retry, treating them as separate orders. The client thinks they're being safe, but they're actually creating duplicates.

I decided on a strict policy:
1. The `Idempotency-Key` header is **required** for all order requests.
2. The key must be a **valid UUID**. Anything else gets a `400 Bad Request`.

This forces clients to be explicit participants in the idempotency contract. If they can't provide a valid key, they don't get to place an order.

```rust
let idempotency_key = match headers.get("idempotency-key") {
    Some(value) => {
        let s = value.to_str().map_err(|_| {
            ApiError::bad_request("Idempotency-Key must be valid UTF-8".to_string())
        })?;
        
        uuid::Uuid::parse_str(s.trim())
            .map(|u| u.to_string())
            .map_err(|_| {
                ApiError::bad_request("Invalid Idempotency-Key format. Expected UUID.".to_string())
            })?
    }
    None => {
        return Err(ApiError::bad_request(
            "Idempotency-Key header is required for all order requests".to_string(),
        ));
    }
};
```

---

## Implementation Challenge: Response Consistency

The async processing model from Phase 4 introduced a subtle problem. When a client makes a request, we generate a random `order_id` and return it in the `202 Accepted` response. If the client retries the same request, they get a *different* random `order_id`, even though the background worker will eventually deduplicate them in the database.

This creates a polling nightmare. Which URL should the client poll? The first `order_id`? The second? They have no way to know which one actually "won" the race.

### Deterministic ID Generation

The solution was to make the `order_id` deterministic. Instead of `Uuid::new_v4()`, I switched to **UUID v5**, which generates a UUID by hashing a namespace and a value.

```rust
let namespace = Uuid::from_u128(0x6ba7b810_9dad_11d1_80b4_00c04fd430c8);
let order_id = Uuid::new_v5(&namespace, idempotency_key.as_bytes());
```

Now, the same `Idempotency-Key` always produces the same `order_id`. If a client retries 10 times, they get the exact same ID in every `202 Accepted` response, and they can poll the same status URL predictably.

This required threading the `order_id` through the entire flow:
1. Generate `order_id` deterministically in the HTTP handler.
2. Add `order_id` to `CreateOrderCommand`.
3. Update `Order::new()` to accept the pre-generated `id`.
4. Pass the `order_id` to the background worker.

---

## Implementation: Defensive Deduplication

The logic layer now implements a three-step defensive pattern to handle idempotency:

### 1. Pre-Check

Before doing any expensive work (locking inventory, checking stock), I check if an order with this `idempotency_key` already exists:

```rust
if let Some(existing_order) = order_repo
    .find_by_idempotency_key(conn, &command.idempotency_key)
    .await
    .map_err(AppError::from)?
{
    tracing::debug!(
        "Order with idempotency_key {} already exists, returning existing order",
        command.idempotency_key
    );
    return Ok(existing_order);
}
```

If it exists, return it immediately. This handles sequential retries efficiently.

### 2. Optimistic Processing

If no order exists, proceed with the normal flow:
- Lock the flash sale row.
- Check inventory.
- Decrement inventory.
- Create the order.

### 3. Race Condition Handling

If two identical requests hit the background worker at the exact same time, they both pass the pre-check. But when they try to insert, the database's `UNIQUE(idempotency_key)` constraint will reject one of them.

I catch this conflict and re-query:

```rust
let saved_order = match order_repo.save(conn, &order).await {
    Ok(order) => order,
    Err(RepoError::Conflict { .. }) => {
        tracing::warn!(
            "Unique constraint violation on idempotency_key: {}, re-querying existing order",
            command.idempotency_key
        );
        
        order_repo
            .find_by_idempotency_key(conn, &command.idempotency_key)
            .await
            .map_err(AppError::from)?
            .ok_or_else(|| RepoError::NotFound {
                entity_type: "Order",
            })?
    }
    Err(e) => return Err(AppError::from(e)),
};
```

This ensures that even in a race condition, both requests return the same order.

---

## Database Schema

I added a migration to include the `idempotency_key` column:

```sql
ALTER TABLE orders ADD COLUMN idempotency_key TEXT NOT NULL DEFAULT gen_random_uuid()::text;
CREATE UNIQUE INDEX idx_orders_idempotency_key ON orders(idempotency_key);
ALTER TABLE orders ALTER COLUMN idempotency_key DROP DEFAULT;
```

The `gen_random_uuid()` default handles existing rows during migration. After the migration, the default is dropped, and the column becomes explicitly managed.

---

## Load Testing: Simulating the Retry Storm

I updated the k6 script to simulate a realistic retry scenario. 20% of all requests reuse an idempotency key from a pool of recently successful requests:

```javascript
const shouldRetry = Math.random() < 0.2;
const idempotencyKey = shouldRetry && retryPool.length > 0
    ? retryPool[Math.floor(Math.random() * retryPool.length)]
    : crypto.randomUUID();
```

### Results

Running 150 VUs for 30 seconds:

| Metric              | Value             |
| :------------------ | :---------------- |
| **HTTP Requests**   | 9,102 total       |
| **Request Rate**    | 297.5 req/s       |
| **Median Duration** | 2.07ms            |
| **P95 Duration**    | 9.67ms            |
| **Acceptance Rate** | 97% (4,500/4,601) |
| **Queue Overflow**  | 3% (101/4,601)    |

**Database Verification**: Zero duplicate orders. Every `idempotency_key` in the database is unique.

**Response Consistency**: Retried requests returned the exact same `order_id` as the original request.

The 3% queue overflow rate is expected and healthy. It shows the system is properly protecting itself under load by rejecting requests that exceed queue capacity with `503 Service Unavailable`, rather than accepting everything and crashing.

## What's Next

The system is fast (Phase 4) and correct (Phase 5). But we're still processing orders through a single background worker, and every order creation locks the same inventory row.

Phase 6 will be about **Inventory Scaling**:
1. **Sharded Inventory**: Breaking inventory into multiple rows to reduce lock contention.
2. **Parallel Workers**: Scaling the background processing to handle higher throughput.

Responsiveness? Check. Correctness? Check. Now let's hit **Scale**.
