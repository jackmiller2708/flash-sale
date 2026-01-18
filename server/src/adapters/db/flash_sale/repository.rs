use anyhow::Context;
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::flash_sale::FlashSale;
use crate::ports::flash_sale_repo::FlashSaleRepo;

pub struct PostgresFlashSaleRepo {
    pool: PgPool,
}

impl PostgresFlashSaleRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl FlashSaleRepo for PostgresFlashSaleRepo {
    async fn find_by_id_with_lock(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        id: Uuid,
    ) -> anyhow::Result<Option<FlashSale>> {
        // SELECT ... FOR UPDATE is key here
        let rec = sqlx::query_as!(
            FlashSale,
            r#"
            SELECT id, product_id, start_time, end_time, total_inventory, 
                   remaining_inventory, per_user_limit, created_at
            FROM flash_sales
            WHERE id = $1
            FOR UPDATE
            "#,
            id
        )
        .fetch_optional(&mut **tx)
        .await
        .context("Failed to fetch flash sale with lock")?;

        Ok(rec)
    }

    async fn update(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        flash_sale: &FlashSale,
    ) -> anyhow::Result<FlashSale> {
        let rec = sqlx::query_as!(
            FlashSale,
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
        .fetch_one(&mut **tx)
        .await
        .context("Failed to update flash sale")?;

        Ok(rec)
    }
}
