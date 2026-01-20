use crate::{domain::flash_sale::FlashSale, errors::RepoError};
use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

#[async_trait]
pub trait FlashSaleRepo: Send + Sync {
    async fn find_by_id_with_lock(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> Result<Option<FlashSale>, RepoError>;
    async fn update(
        &self,
        conn: &mut PgConnection,
        flash_sale: &FlashSale,
    ) -> Result<FlashSale, RepoError>;
}
