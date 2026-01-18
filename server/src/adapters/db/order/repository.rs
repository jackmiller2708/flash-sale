use anyhow::Context;
use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::order::{Order, OrderStatus};
use crate::ports::order_repo::OrderRepo;

pub struct PostgresOrderRepo {
    pool: PgPool,
}

impl PostgresOrderRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl OrderRepo for PostgresOrderRepo {
    async fn save(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        order: &Order,
    ) -> anyhow::Result<Order> {
        let rec = sqlx::query_as!(
            Order,
            r#"
            INSERT INTO orders (id, user_id, flash_sale_id, quantity, status, created_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, user_id, flash_sale_id, quantity, status as "status: OrderStatus", created_at
            "#,
            order.id,
            order.user_id,
            order.flash_sale_id,
            order.quantity,
            order.status as OrderStatus,
            order.created_at
        )
        .fetch_one(&mut **tx)
        .await
        .context("Failed to save order")?;

        Ok(rec)
    }
}
