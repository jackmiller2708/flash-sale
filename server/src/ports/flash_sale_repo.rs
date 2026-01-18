use crate::domain::flash_sale::FlashSale;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait FlashSaleRepo: Send + Sync {
    async fn find_by_id_with_lock(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        id: Uuid,
    ) -> anyhow::Result<Option<FlashSale>>;
    async fn update(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        flash_sale: &FlashSale,
    ) -> anyhow::Result<FlashSale>;
}
