use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use crate::domain::order::{Order, OrderStatus};

#[derive(Debug, FromRow)]
pub struct OrderRecord {
    pub id: Uuid,
    pub user_id: Uuid,
    pub flash_sale_id: Uuid,
    pub quantity: i32,
    pub status: OrderStatus,
    pub idempotency_key: String,
    pub created_at: DateTime<Utc>,
}

impl From<Order> for OrderRecord {
    fn from(order: Order) -> Self {
        Self {
            id: order.id,
            user_id: order.user_id,
            flash_sale_id: order.flash_sale_id,
            quantity: order.quantity,
            status: order.status,
            idempotency_key: order.idempotency_key,
            created_at: order.created_at,
        }
    }
}
