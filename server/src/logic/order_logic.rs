use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    adapters::http::dtos::order_dto::CreateOrderRequest,
    domain::order::Order,
    errors::{AppError, RepoError, ServiceError},
    ports::{FlashSaleRepo, OrderRepo},
};

#[derive(Debug, Clone)]
pub struct CreateOrderCommand {
    pub user_id: Uuid,
    pub flash_sale_id: Uuid,
    pub quantity: i32,
}

impl From<CreateOrderRequest> for CreateOrderCommand {
    fn from(req: CreateOrderRequest) -> Self {
        Self {
            user_id: req.user_id,
            flash_sale_id: req.flash_sale_id,
            quantity: req.quantity,
        }
    }
}

pub async fn create_order<FR: FlashSaleRepo + ?Sized, OR: OrderRepo + ?Sized>(
    conn: &mut PgConnection,
    flash_sale_repo: &FR,
    order_repo: &OR,
    command: CreateOrderCommand,
) -> Result<Order, AppError> {
    // 1. Fetch Flash Sale with Lock
    let flash_sale = flash_sale_repo
        .find_by_id_with_lock(conn, command.flash_sale_id)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| RepoError::NotFound {
            entity_type: "FlashSale",
        })?;

    // 2. Check Inventory
    if flash_sale.remaining_inventory < command.quantity {
        return Err(ServiceError::Conflict("sold out".to_string()).into());
    }

    // 3. Check if active
    if !flash_sale.is_active() {
        return Err(ServiceError::BusinessRule("flash sale is not active".to_string()).into());
    }

    // 4. Decrement Inventory
    let mut updated_flash_sale = flash_sale.clone();
    updated_flash_sale.remaining_inventory -= command.quantity;

    flash_sale_repo
        .update(conn, &updated_flash_sale)
        .await
        .map_err(AppError::from)?;

    // 5. Create Order
    let order = Order::new(command.user_id, command.flash_sale_id, command.quantity);

    let saved_order = order_repo
        .save(conn, &order)
        .await
        .map_err(AppError::from)?;

    Ok(saved_order)
}
