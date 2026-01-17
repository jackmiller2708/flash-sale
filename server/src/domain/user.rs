use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::adapters::db::user::UserRecord;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
}

impl From<UserRecord> for User {
    fn from(value: UserRecord) -> Self {
        Self {
            id: value.id,
            created_at: value.created_at,
        }
    }
}
