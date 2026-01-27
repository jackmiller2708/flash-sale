use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    domain::order::Order,
    errors::{AppError, RepoError, ServiceError},
    ports::{FlashSaleRepo, OrderRepo},
};

#[derive(Debug, Clone)]
pub struct CreateOrderCommand {
    pub order_id: Uuid,
    pub user_id: Uuid,
    pub flash_sale_id: Uuid,
    pub quantity: i32,
    pub idempotency_key: String,
}

pub async fn create_order<FR: FlashSaleRepo + ?Sized, OR: OrderRepo + ?Sized>(
    conn: &mut PgConnection,
    flash_sale_repo: &FR,
    order_repo: &OR,
    command: CreateOrderCommand,
) -> Result<Order, AppError> {
    // 1. Check for existing order with same idempotency key (Idempotent Response)
    if let Some(existing_order) = order_repo
        .find_by_idempotency_key(conn, &command.idempotency_key)
        .await
        .map_err(AppError::from)?
    {
        tracing::debug!(
            "Idempotent request detected: returning existing order {}",
            existing_order.id
        );
        return Ok(existing_order);
    }

    // 2. Fetch Flash Sale with Lock
    let flash_sale = flash_sale_repo
        .find_by_id_with_lock(conn, command.flash_sale_id)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| RepoError::NotFound {
            entity_type: "FlashSale",
        })?;

    // 3. Check Inventory
    if flash_sale.remaining_inventory < command.quantity {
        return Err(ServiceError::Conflict("sold out".to_string()).into());
    }

    // 4. Check if active
    if !flash_sale.is_active() {
        return Err(ServiceError::BusinessRule("flash sale is not active".to_string()).into());
    }

    // 5. Decrement Inventory
    let mut updated_flash_sale = flash_sale.clone();
    updated_flash_sale.remaining_inventory -= command.quantity;

    flash_sale_repo
        .update(conn, &updated_flash_sale)
        .await
        .map_err(AppError::from)?;

    // 6. Create Order with idempotency key
    let order = Order::new(
        command.user_id,
        command.flash_sale_id,
        command.quantity,
        command.idempotency_key.clone(),
    );

    // 7. Save order (handle race condition on unique constraint)
    let saved_order = match order_repo.save(conn, &order).await {
        Ok(order) => order,
        Err(RepoError::Conflict { .. }) => {
            // Race condition: another request with same idempotency key succeeded
            // Re-query to get the existing order
            tracing::warn!(
                "Unique constraint violation on idempotency_key: {}, re-querying existing order",
                command.idempotency_key
            );

            order_repo
                .find_by_idempotency_key(conn, &command.idempotency_key)
                .await
                .map_err(AppError::from)?
                .ok_or_else(|| RepoError::NotFound {
                    entity_type: "Order",
                })?
        }
        Err(e) => return Err(AppError::from(e)),
    };

    Ok(saved_order)
}
