use metrics_exporter_prometheus::PrometheusHandle;
use std::sync::Arc;

use crate::ports::{
    flash_sale_repo::FlashSaleRepo, order_repo::OrderRepo, product_repo::ProductRepo,
    user_repo::UserRepo,
};

#[derive(Clone)]
pub struct AppState {
    pub user_repo: Arc<dyn UserRepo>,
    pub product_repo: Arc<dyn ProductRepo>,
    pub flash_sale_repo: Arc<dyn FlashSaleRepo>,
    pub order_repo: Arc<dyn OrderRepo>,
    pub db_pool: sqlx::PgPool,
    pub prometheus_handle: PrometheusHandle,
}

impl AppState {
    pub fn db_pool(&self) -> &sqlx::PgPool {
        &self.db_pool
    }
}
