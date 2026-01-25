use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::order::Order;

#[derive(Debug, Deserialize)]
pub struct CreateOrderRequest {
    pub user_id: Uuid,
    pub flash_sale_id: Uuid,
    pub quantity: i32,
}

#[derive(Debug, Serialize)]
pub struct OrderResponse {
    pub order_id: Uuid,
    pub status: String,
}

impl From<Order> for OrderResponse {
    fn from(order: Order) -> Self {
        Self {
            order_id: order.id,
            status: format!("{:?}", order.status).to_uppercase(),
        }
    }
}

/// Response when an order is accepted for async processing
#[derive(Debug, Serialize)]
pub struct OrderAcceptedResponse {
    pub order_id: Uuid,
    pub status: String,
    pub status_url: String,
}

/// Response for order status polling
#[derive(Debug, Serialize)]
pub struct OrderStatusResponse {
    pub order_id: Uuid,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<OrderResult>,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum OrderResult {
    Success(OrderResponse),
    Error { message: String },
}
