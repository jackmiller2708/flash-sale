-- ===============================
-- FlashDeal v1 Database Setup
-- ===============================

-- Enable extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- ===============================
-- Users
-- ===============================
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Phase 1 note:
-- No email / auth fields yet.
-- Users are identity tokens only.

-- ===============================
-- Products
-- ===============================
CREATE TABLE products (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- ===============================
-- Flash Sales
-- ===============================
CREATE TABLE flash_sales (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
    product_id UUID NOT NULL REFERENCES products (id),
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ NOT NULL,
    total_inventory INTEGER NOT NULL CHECK (total_inventory >= 0),
    remaining_inventory INTEGER NOT NULL CHECK (remaining_inventory >= 0),
    per_user_limit INTEGER NOT NULL CHECK (per_user_limit > 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (end_time > start_time),
    CHECK (
        remaining_inventory <= total_inventory
    )
);

-- Index to support time-based reads
CREATE INDEX idx_flash_sales_time ON flash_sales (start_time, end_time);

-- ===============================
-- Orders
-- ===============================
CREATE TYPE order_status AS ENUM (
    'PENDING',
    'CONFIRMED',
    'FAILED'
);

CREATE TABLE orders (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
    user_id UUID NOT NULL REFERENCES users (id),
    flash_sale_id UUID NOT NULL REFERENCES flash_sales (id),
    quantity INTEGER NOT NULL CHECK (quantity > 0),
    status order_status NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Supports per-user purchase checks (Phase 1 optional, Phase 5 required)
CREATE INDEX idx_orders_user_flash_sale ON orders (user_id, flash_sale_id);

-- Not used in Phase 1, but harmless and future-aligned
CREATE INDEX idx_orders_status ON orders (status);