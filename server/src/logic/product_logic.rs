use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    adapters::http::dtos::product_dto::CreateProductRequest, domain::product::Product,
    errors::AppError, ports::ProductRepo,
};

#[derive(Debug, Clone)]
pub struct CreateProductCommand {
    pub id: Uuid,
    pub name: String,
}

impl TryFrom<CreateProductRequest> for CreateProductCommand {
    type Error = AppError;

    fn try_from(value: CreateProductRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Uuid::new_v4(),
            name: value.name,
        })
    }
}

pub async fn save_product<R: ProductRepo + ?Sized>(
    conn: &mut PgConnection,
    repo: &R,
    command: CreateProductCommand,
) -> Result<Product, AppError> {
    let product = Product::try_from(command)?;
    repo.save(conn, product).await.map_err(Into::into)
}

pub async fn get_products<R: ProductRepo + ?Sized>(
    conn: &mut PgConnection,
    repo: &R,
) -> Result<Vec<Product>, AppError> {
    repo.get_all(conn).await.map_err(Into::into)
}
