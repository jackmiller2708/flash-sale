use async_trait::async_trait;
use sqlx::PgConnection;

use crate::{
    adapters::db::error_mapper::map_sqlx_error,
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
        .fetch_one(conn)
        .await
        .map_err(|e| map_sqlx_error(e, "save_order", "order"))?;

        Ok(rec)
    }
}
