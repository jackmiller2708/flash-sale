use async_trait::async_trait;
use sqlx::PgConnection;

use crate::{
    adapters::db::{error_mapper::map_sqlx_error, product::ProductRecord},
    domain::product::Product,
    errors::RepoError,
    ports::ProductRepo,
};

pub struct PostgresProductRepo;

impl PostgresProductRepo {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl ProductRepo for PostgresProductRepo {
    async fn save(&self, conn: &mut PgConnection, product: Product) -> Result<Product, RepoError> {
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
        .fetch_one(conn)
        .await
        .map_err(|e| map_sqlx_error(e, "save_product", "product"))?;

        Product::try_from(saved_record).map_err(|e| RepoError::Database {
            operation: "convert_product_record",
            source: sqlx::Error::Decode(Box::new(e)),
        })
    }

    async fn get_all(&self, conn: &mut PgConnection) -> Result<Vec<Product>, RepoError> {
        let records = sqlx::query_as!(
            ProductRecord,
            r#"
            SELECT id, name, created_at
            FROM products
            "#
        )
        .fetch_all(conn)
        .await
        .map_err(|e| map_sqlx_error(e, "get_all_products", "product"))?;

        records
            .into_iter()
            .map(Product::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| RepoError::Database {
                operation: "convert_product_records",
                source: sqlx::Error::Decode(Box::new(e)),
            })
    }

    async fn find_by_id(
        &self,
        conn: &mut PgConnection,
        id: uuid::Uuid,
    ) -> Result<Option<Product>, RepoError> {
        let record = sqlx::query_as!(
            ProductRecord,
            r#"
            SELECT id, name, created_at
            FROM products
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(conn)
        .await
        .map_err(|e| map_sqlx_error(e, "find_product_by_id", "product"))?;

        record
            .map(Product::try_from)
            .transpose()
            .map_err(|e| RepoError::Database {
                operation: "convert_product_record",
                source: sqlx::Error::Decode(Box::new(e)),
            })
    }
}
