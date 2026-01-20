use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    adapters::db::{error_mapper::map_sqlx_error, flash_sale::FlashSaleRecord},
    domain::flash_sale::FlashSale,
    errors::RepoError,
    ports::flash_sale_repo::FlashSaleRepo,
};

pub struct PostgresFlashSaleRepo;

impl PostgresFlashSaleRepo {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl FlashSaleRepo for PostgresFlashSaleRepo {
    async fn find_by_id_with_lock(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> Result<Option<FlashSale>, RepoError> {
        // SELECT ... FOR UPDATE is key here
        let record = sqlx::query_as!(
            FlashSaleRecord,
            r#"
            SELECT id, product_id, start_time, end_time, total_inventory, 
                   remaining_inventory, per_user_limit, created_at
            FROM flash_sales
            WHERE id = $1
            FOR UPDATE
            "#,
            id
        )
        .fetch_optional(conn)
        .await
        .map_err(|e| map_sqlx_error(e, "find_flash_sale_with_lock", "flash_sale"))?;

        Ok(record.map(FlashSale::from))
    }

    async fn update(
        &self,
        conn: &mut PgConnection,
        flash_sale: &FlashSale,
    ) -> Result<FlashSale, RepoError> {
        let saved_record = sqlx::query_as!(
            FlashSaleRecord,
            r#"
            UPDATE flash_sales
            SET remaining_inventory = $2
            WHERE id = $1
            RETURNING id, product_id, start_time, end_time, total_inventory, 
                      remaining_inventory, per_user_limit, created_at
            "#,
            flash_sale.id,
            flash_sale.remaining_inventory
        )
        .fetch_one(conn)
        .await
        .map_err(|e| map_sqlx_error(e, "update_flash_sale", "flash_sale"))?;

        Ok(saved_record.into())
    }
}
