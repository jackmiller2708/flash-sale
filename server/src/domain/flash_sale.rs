use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlashSale {
    pub id: Uuid,
    pub product_id: Uuid,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub total_inventory: i32,
    pub remaining_inventory: i32,
    pub per_user_limit: i32,
    pub created_at: DateTime<Utc>,
}

impl FlashSale {
    pub fn is_active(&self) -> bool {
        let now = Utc::now();
        now >= self.start_time && now <= self.end_time
    }

    pub fn is_sold_out(&self) -> bool {
        self.remaining_inventory <= 0
    }
}
