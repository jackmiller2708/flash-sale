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
        let saved_record = sqlx::query_as!(
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

        let saved_product = Product::try_from(saved_record)?;

        Ok(saved_product)
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

        let products: Result<Vec<Product>, _> =
            records.into_iter().map(Product::try_from).collect();

        Ok(products?)
    }
}
