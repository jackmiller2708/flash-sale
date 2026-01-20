use crate::{domain::order::Order, errors::RepoError};
use async_trait::async_trait;
use sqlx::PgConnection;

#[async_trait]
pub trait OrderRepo: Send + Sync {
    async fn save(&self, conn: &mut PgConnection, order: &Order) -> Result<Order, RepoError>;
}
