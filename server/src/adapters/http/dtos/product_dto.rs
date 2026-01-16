use chrono::{DateTime, Utc};

use crate::domain::product::Product;

#[derive(Debug, serde::Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
}

#[derive(serde::Serialize)]
pub struct ProductResponse {
    pub id: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

impl From<Product> for ProductResponse {
    fn from(product: Product) -> Self {
        Self {
            id: product.id.to_string(),
            name: product.name,
            created_at: product.created_at,
        }
    }
}
