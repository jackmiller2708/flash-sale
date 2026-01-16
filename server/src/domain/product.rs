use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    adapters::db::product::record::ProductRecord, logic::product_logic::CreateProductCommand,
};

#[derive(Debug, Clone)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

impl From<CreateProductCommand> for Product {
    fn from(value: CreateProductCommand) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: value.name,
            created_at: DateTime::default(),
        }
    }
}

impl From<ProductRecord> for Product {
    fn from(value: ProductRecord) -> Self {
        Self {
            id: value.id,
            name: value.name,
            created_at: value.created_at,
        }
    }
}
