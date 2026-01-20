# Devlog 02: Naive Concurrency

## Context
Phase 1 involved implementing the core purchase flow. The "naive" part refers to using a single row for inventory and locking it during the entire transaction.

## Lessons Learned

### Pessimistic Locking with `FOR UPDATE`
By using `SELECT ... FOR UPDATE`, we ensure that only one transaction can modify a specific flash sale at a time.
- **Benefit**: Guaranteed correctness. No overselling is possible because the row is locked.
- **Cost**: Throughput is strictly limited by the latency of the transaction. If a transaction takes 50ms, the system can only handle 20 requests/sec for that specific sale.

### Transaction Management
In Rust, managing `sqlx::Transaction` across different functions requires passing a mutable reference to the connection.
- **Pattern**: `&mut PgConnection` is the most flexible way to pass either a connection or a transaction to repository methods.

### Separation of Concerns
Keeping the transaction boundary in the handler/service layer while repositories focus on single SQL operations makes the code cleaner, but requires passing the executor around.

## Observations
Without load testing, the performance bottleneck is theoretical. Implementing Phase 2 metrics is essential to see this bottleneck in action.

## Initial Load Test Findings
The first baseline load test yielded the following results:
- **Throughput**: ~91.3 RPS (Requests Per Second) with 10 concurrent users.
- **Latency**:
    - **Average**: 8.4ms
    - **P95**: 13.36ms
    - **Max**: 124ms
- **Analysis**:
    - The throughput is limited by the 100ms sleep in the test script (`91 RPS` is roughly `10 users / 0.11s`).
    - The `100% http_req_failed` rate in k6 is expected behavior because the check `is status 201 or 409 or 404` passed 100%, but k6 marks 4xx as failures by default.
    - Low latency confirms that the "Naive" approach performs well under light concurrency (10 VUs), but we expect this to degrade as we increase contention.
