use tokio::sync::{mpsc, oneshot};
use tracing::{error, info, warn};

use crate::{
    domain::order::Order,
    errors::AppError,
    logic::order_logic::{CreateOrderCommand, create_order},
    ports::{FlashSaleRepo, OrderRepo},
};

/// Message type for the order queue
#[derive(Debug)]
pub struct OrderQueueMessage {
    pub command: CreateOrderCommand,
    pub response_tx: oneshot::Sender<Result<Order, AppError>>,
}

/// Create and spawn the order queue worker
///
/// Returns the sender half of the channel that handlers can use to enqueue orders
pub fn spawn_order_queue_worker(
    db_pool: sqlx::PgPool,
    flash_sale_repo: std::sync::Arc<dyn FlashSaleRepo>,
    order_repo: std::sync::Arc<dyn OrderRepo>,
    queue_capacity: usize,
) -> mpsc::Sender<OrderQueueMessage> {
    let (tx, mut rx) = mpsc::channel::<OrderQueueMessage>(queue_capacity);

    tokio::spawn(async move {
        info!(
            "Order queue worker started with capacity {}",
            queue_capacity
        );

        while let Some(msg) = rx.recv().await {
            let OrderQueueMessage {
                command,
                response_tx,
            } = msg;

            // Record queue depth metric
            metrics::gauge!("order_queue_depth").set(rx.len() as f64);

            // Process the order
            let mut conn = match db_pool.acquire().await {
                Ok(conn) => conn,
                Err(e) => {
                    error!("Failed to acquire DB connection: {:?}", e);
                    let _ = response_tx.send(Err(AppError::Unexpected(e.into())));
                    continue;
                }
            };

            let result = create_order(
                &mut *conn,
                flash_sale_repo.as_ref(),
                order_repo.as_ref(),
                command,
            )
            .await;

            // Send result back to handler
            if response_tx.send(result).is_err() {
                warn!("Failed to send response - handler may have timed out");
            }
        }

        info!("Order queue worker shutting down");
    });

    tx
}
