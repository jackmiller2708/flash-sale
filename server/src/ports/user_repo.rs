use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{domain::user::User, errors::RepoError};

#[async_trait]
pub trait UserRepo: Send + Sync {
    async fn save(&self, conn: &mut PgConnection, user: User) -> Result<User, RepoError>;
    async fn get_all(&self, conn: &mut PgConnection) -> Result<Vec<User>, RepoError>;
    async fn get_by_id(&self, conn: &mut PgConnection, id: Uuid) -> Result<User, RepoError>;
}
