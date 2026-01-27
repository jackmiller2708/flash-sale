-- Phase 5: Add idempotency_key to orders table
-- This enables retry-safe order creation

-- Add idempotency_key column
ALTER TABLE orders
ADD COLUMN idempotency_key TEXT NOT NULL DEFAULT gen_random_uuid()::TEXT;

-- Create unique constraint to prevent duplicate orders
ALTER TABLE orders
ADD CONSTRAINT orders_idempotency_key_unique UNIQUE (idempotency_key);

-- Create index for fast lookups by idempotency key
CREATE INDEX idx_orders_idempotency_key ON orders (idempotency_key);

-- Remove the temporary default (future inserts must provide explicit value)
ALTER TABLE orders
ALTER COLUMN idempotency_key DROP DEFAULT;
