use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct UserRecord {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
}
