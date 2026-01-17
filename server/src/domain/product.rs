use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    adapters::db::product::ProductRecord,
    errors::{AppError, DomainError},
    logic::CreateProductCommand,
};

#[derive(Debug, Clone)]
pub struct ProductName(String);

impl ProductName {
    pub fn new(value: String) -> Result<Self, AppError> {
        if value.trim().is_empty() {
            return Err(AppError::Domain(DomainError::ProductNameEmpty));
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct Product {
    pub id: Uuid,
    pub name: ProductName,
    pub created_at: DateTime<Utc>,
}

impl TryFrom<CreateProductCommand> for Product {
    type Error = AppError;

    fn try_from(value: CreateProductCommand) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Uuid::new_v4(),
            name: ProductName::new(value.name)?,
            created_at: DateTime::default(),
        })
    }
}

impl TryFrom<ProductRecord> for Product {
    type Error = AppError;

    fn try_from(value: ProductRecord) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            name: ProductName::new(value.name)?,
            created_at: value.created_at,
        })
    }
}
