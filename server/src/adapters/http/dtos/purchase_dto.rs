use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct PurchaseRequest {
    pub flash_sale_id: Uuid,
    pub quantity: i32,
}

#[derive(Debug, Serialize)]
pub struct PurchaseResponse {
    pub order_id: Uuid,
    pub status: String,
}
