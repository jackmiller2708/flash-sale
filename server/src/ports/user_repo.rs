use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::User;

#[async_trait]
pub trait UserRepo: Send + Sync {
    async fn save(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        user: User,
    ) -> anyhow::Result<User>;
    async fn get_all(&self) -> anyhow::Result<Vec<User>>;
    async fn get_by_id(&self, id: Uuid) -> anyhow::Result<User>;
}
