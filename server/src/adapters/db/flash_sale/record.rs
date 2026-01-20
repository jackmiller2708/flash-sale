use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use crate::domain::flash_sale::FlashSale;

#[derive(Debug, FromRow)]
pub struct FlashSaleRecord {
    pub id: Uuid,
    pub product_id: Uuid,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub total_inventory: i32,
    pub remaining_inventory: i32,
    pub per_user_limit: i32,
    pub created_at: DateTime<Utc>,
}

impl From<FlashSale> for FlashSaleRecord {
    fn from(value: FlashSale) -> Self {
        Self {
            id: value.id,
            product_id: value.product_id,
            start_time: value.start_time,
            end_time: value.end_time,
            total_inventory: value.total_inventory,
            remaining_inventory: value.remaining_inventory,
            per_user_limit: value.per_user_limit,
            created_at: value.created_at,
        }
    }
}
