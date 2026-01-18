use async_trait::async_trait;

use crate::domain::Product;

#[async_trait]
pub trait ProductRepo: Send + Sync {
    async fn save(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        product: Product,
    ) -> anyhow::Result<Product>;
    async fn get_all(&self) -> anyhow::Result<Vec<Product>>;
}
