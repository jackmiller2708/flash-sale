-- ===============================
-- Seed data for Phase 1 Testing
-- ===============================
-- 1. Create 100 users for concurrency testing
INSERT INTO users (id)
SELECT uuid_generate_v4()
FROM generate_series(1, 100);
-- 2. Create a test product
INSERT INTO products (id, name)
VALUES (
        '00000000-0000-0000-0000-000000000001',
        'Pro Gaming Laptop'
    );
-- 3. Create a Flash Sale
-- Starts in the past, ends in 24 hours
-- 50 units total, 1 per user
INSERT INTO flash_sales (
        id,
        product_id,
        start_time,
        end_time,
        total_inventory,
        remaining_inventory,
        per_user_limit
    )
VALUES (
        '11111111-1111-1111-1111-111111111111',
        '00000000-0000-0000-0000-000000000001',
        now() - interval '1 hour',
        now() + interval '24 hours',
        50,
        50,
        1
    );