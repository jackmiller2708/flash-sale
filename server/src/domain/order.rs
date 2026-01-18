use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "order_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    Pending,
    Confirmed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: Uuid,
    pub user_id: Uuid,
    pub flash_sale_id: Uuid,
    pub quantity: i32,
    pub status: OrderStatus,
    pub created_at: DateTime<Utc>,
}

impl Order {
    pub fn new(user_id: Uuid, flash_sale_id: Uuid, quantity: i32) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            flash_sale_id,
            quantity,
            status: OrderStatus::Pending,
            created_at: Utc::now(),
        }
    }
}
