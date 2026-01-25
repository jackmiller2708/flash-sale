use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{error, info};
use uuid::Uuid;

use crate::{
    domain::order::OrderProcessingStatus,
    logic::order_logic::{CreateOrderCommand, create_order},
    ports::{FlashSaleRepo, OrderRepo},
};

/// Message type for the order queue
#[derive(Debug)]
pub struct OrderQueueMessage {
    pub order_id: Uuid,
    pub command: CreateOrderCommand,
}

/// Create and spawn the order queue worker
///
/// Returns the sender half of the channel that handlers can use to enqueue orders
pub fn spawn_order_queue_worker(
    db_pool: sqlx::PgPool,
    flash_sale_repo: Arc<dyn FlashSaleRepo>,
    order_repo: Arc<dyn OrderRepo>,
    order_status_store: Arc<dashmap::DashMap<Uuid, OrderProcessingStatus>>,
    queue_capacity: usize,
) -> mpsc::Sender<OrderQueueMessage> {
    let (tx, mut rx) = mpsc::channel::<OrderQueueMessage>(queue_capacity);

    tokio::spawn(async move {
        info!(
            "Order queue worker started with capacity {}",
            queue_capacity
        );

        while let Some(msg) = rx.recv().await {
            let OrderQueueMessage { order_id, command } = msg;

            // Record queue depth metric
            metrics::gauge!("order_queue_depth").set(rx.len() as f64);

            // Process the order
            let mut tx = match db_pool.begin().await {
                Ok(conn) => conn,
                Err(e) => {
                    error!(order_id = %order_id, error = ?e, "Failed to acquire DB connection");
                    // Update status to failed
                    order_status_store.insert(
                        order_id,
                        OrderProcessingStatus::Failed(format!("Database connection failed: {}", e)),
                    );
                    continue;
                }
            };

            let result = create_order(
                &mut *tx,
                flash_sale_repo.as_ref(),
                order_repo.as_ref(),
                command,
            )
            .await;

            let commit_result = tx.commit().await;

            if commit_result.is_err() {
                error!(order_id = %order_id, error = ?commit_result, "Failed to commit transaction");
                // Update status to failed
                order_status_store.insert(
                    order_id,
                    OrderProcessingStatus::Failed(format!(
                        "Transaction commit failed: {}",
                        commit_result.unwrap_err()
                    )),
                );
                continue;
            }

            // Store result in status store
            let status = match result {
                Ok(order) => {
                    info!(order_id = %order_id, "Order processed successfully");
                    OrderProcessingStatus::Completed(order)
                }
                Err(e) => {
                    info!(order_id = %order_id, error = ?e, "Order processing failed");
                    OrderProcessingStatus::Failed(e.to_string())
                }
            };

            order_status_store.insert(order_id, status);
        }

        info!("Order queue worker shutting down");
    });

    tx
}
