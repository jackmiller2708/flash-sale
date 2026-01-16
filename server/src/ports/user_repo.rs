use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::user::User;

#[async_trait]
pub trait UserRepo: Send + Sync {
    async fn save(&self, user: User) -> anyhow::Result<User>;
    async fn get_all(&self) -> anyhow::Result<Vec<User>>;
    async fn get_by_id(&self, id: Uuid) -> anyhow::Result<User>;
}
