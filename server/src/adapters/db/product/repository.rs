use async_trait::async_trait;
use sqlx::PgPool;

use crate::{
    adapters::db::product::record::ProductRecord, domain::product::Product,
    ports::product_repo::ProductRepo,
};

pub struct PostgresProductRepo {
    pool: PgPool,
}

impl PostgresProductRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProductRepo for PostgresProductRepo {
    async fn save(&self, product: Product) -> anyhow::Result<Product> {
        let record = ProductRecord::from(product);
        let saved = sqlx::query_as!(
            ProductRecord,
            r#"
            INSERT INTO products (name)
            VALUES ($1)
            RETURNING id, name, created_at
            "#,
            record.name,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(Product::from(saved))
    }

    async fn get_all(&self) -> anyhow::Result<Vec<Product>> {
        let records = sqlx::query_as!(
            ProductRecord,
            r#"
            SELECT id, name, created_at
            FROM products
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(records.into_iter().map(|r| Product::from(r)).collect())
    }
}
