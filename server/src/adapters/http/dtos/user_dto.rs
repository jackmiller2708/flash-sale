use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::user::User;

#[derive(serde::Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            created_at: user.created_at,
        }
    }
}
