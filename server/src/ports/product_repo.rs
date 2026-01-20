use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{domain::product::Product, errors::RepoError};

#[async_trait]
pub trait ProductRepo: Send + Sync {
    async fn save(&self, conn: &mut PgConnection, product: Product) -> Result<Product, RepoError>;
    async fn get_all(&self, conn: &mut PgConnection) -> Result<Vec<Product>, RepoError>;
    async fn find_by_id(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> Result<Option<Product>, RepoError>;
}
