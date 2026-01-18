use crate::domain::order::Order;
use async_trait::async_trait;

#[async_trait]
pub trait OrderRepo: Send + Sync {
    async fn save(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        order: &Order,
    ) -> anyhow::Result<Order>;
}
