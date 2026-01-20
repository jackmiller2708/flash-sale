# Phase 6: Inventory Scaling Checklists

**Goal:** Reduce lock contention and increase throughput.

## 1. Inventory Sharding

- [ ] Split `total_inventory` into multiple rows (e.g., 10 shards per flash sale) in `inventory_shards` table
- [ ] Logic Update: Randomly select a shard to attempt order
- [ ] If shard is empty/locked, try another shard (or fail fast depending on strategy)

## 2. Optimistic Locking

- [ ] Add `version` column to inventory table
- [ ] Update SQL: `UPDATE inventory SET count = count - 1, version = version + 1 WHERE id = $1 AND version = $2`
- [ ] Handle Update count = 0 (Stale Object Error) -> Retry logic

## 3. Benchmarking

- [ ] Run load test again
- [ ] Compare Throughput (RPS) of Sharded/Optimistic vs Single Row Pessimistic
- [ ] Verify Exit Criteria: Throughput increases under load (instead of flattening or crashing)
