# Devlog 02: Naive Concurrency & Pattern Alignment

## Context
In Phase 1, I focused on implementing the core order flow. The "naive" part refers to using a single row for inventory and locking it during the entire transaction. During this phase, I also aligned the project's architectural patterns to ensure consistency as I scale.

## Lessons Learned

### Terminology Alignment: Purchase vs. Order
I decided to harmonize the terminology between the HTTP layer and the domain logic. 
- **Decision**: Use "Order" instead of "Purchase" to match the `Order` domain object. 
- **Action**: Renamed `purchase_handler` to `order_handler`, `purchase_dto` to `order_dto`, and updated all service-level logic to `order_logic`. The endpoint was moved to `/api/orders`.

### Repository Record Pattern
To decouple my domain objects from the database schema, I adopted a standard "Record" pattern across all repositories.
- **Pattern**: For each repository (e.g., `Order`, `FlashSale`), I introduced a corresponding `*Record` struct (e.g., `OrderRecord`) that implements `sqlx::FromRow`.
- **Flow**: Repositories now query into `Record` structs and then convert them into Domain objects via `From`/`Into` implementations. This allows the domain to evolve independently of the persistence schema.

### Pessimistic Locking with `FOR UPDATE`
By using `SELECT ... FOR UPDATE`, I ensure that only one transaction can modify a specific flash sale at a time.
- **Benefit**: Guaranteed correctness. No overselling is possible because the row is locked.
- **Cost**: Throughput is strictly limited by the latency of the transaction. If a transaction takes 50ms, the system can only handle 20 requests/sec for that specific sale.

### Transaction Management
In Rust, managing `sqlx::Transaction` across different functions requires passing a mutable reference to the connection.
- **Pattern**: `&mut PgConnection` is the most flexible way to pass either a connection or a transaction to repository methods.

### Separation of Concerns
Keeping the transaction boundary in the handler/logic layer while repositories focus on single SQL operations makes the code cleaner, but requires passing the executor around.

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
    - **Correction**: Initial tests hit foreign key violations because of random `user_id` generation. I updated the load test to fetch real users from the `/users` endpoint in the `setup` phase, ensuring valid references.
    - Low latency confirms that the "Naive" approach performs well under light concurrency (10 VUs), but I expect this to degrade as I increase contention.
