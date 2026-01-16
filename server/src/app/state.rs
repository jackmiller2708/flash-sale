use std::sync::Arc;

use crate::ports::{product_repo::ProductRepo, user_repo::UserRepo};

#[derive(Clone)]
pub struct AppState {
    pub user_repo: Arc<dyn UserRepo>,
    pub product_repo: Arc<dyn ProductRepo>,
}
