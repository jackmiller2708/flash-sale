use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::domain::Product;

#[derive(Debug, FromRow)]
pub struct ProductRecord {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

impl From<Product> for ProductRecord {
    fn from(value: Product) -> Self {
        Self {
            id: value.id,
            name: value.name.as_str().to_owned(),
            created_at: value.created_at,
        }
    }
}
