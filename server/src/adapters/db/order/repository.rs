use async_trait::async_trait;
use sqlx::PgConnection;

use crate::{
    adapters::db::{error_mapper::map_sqlx_error, order::OrderRecord},
    domain::order::{Order, OrderStatus},
    errors::RepoError,
    ports::order_repo::OrderRepo,
};

pub struct PostgresOrderRepo;

impl PostgresOrderRepo {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl OrderRepo for PostgresOrderRepo {
    async fn save(&self, conn: &mut PgConnection, order: &Order) -> Result<Order, RepoError> {
        let saved_record = sqlx::query_as!(
            OrderRecord,
            r#"
            INSERT INTO orders (id, user_id, flash_sale_id, quantity, status, idempotency_key, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, user_id, flash_sale_id, quantity, status as "status: OrderStatus", idempotency_key, created_at
            "#,
            order.id,
            order.user_id,
            order.flash_sale_id,
            order.quantity,
            order.status as OrderStatus,
            order.idempotency_key,
            order.created_at
        )
        .fetch_one(conn)
        .await
        .map_err(|e| map_sqlx_error(e, "save_order", "order"))?;

        Ok(saved_record.into())
    }

    async fn find_by_idempotency_key(
        &self,
        conn: &mut PgConnection,
        key: &str,
    ) -> Result<Option<Order>, RepoError> {
        let result = sqlx::query_as!(
            OrderRecord,
            r#"
            SELECT id, user_id, flash_sale_id, quantity, status as "status: OrderStatus", idempotency_key, created_at
            FROM orders
            WHERE idempotency_key = $1
            LIMIT 1
            "#,
            key
        )
        .fetch_optional(conn)
        .await
        .map_err(|e| map_sqlx_error(e, "find_by_idempotency_key", "order"))?;

        Ok(result.map(Into::into))
    }
}
