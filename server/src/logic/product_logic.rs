use uuid::Uuid;

use crate::{
    adapters::http::dtos::product_dto::CreateProductRequest, domain::product::Product,
    ports::product_repo::ProductRepo,
};

#[derive(Debug, Clone)]
pub struct CreateProductCommand {
    pub id: Uuid,
    pub name: String,
}

impl TryFrom<CreateProductRequest> for CreateProductCommand {
    type Error = anyhow::Error;

    fn try_from(value: CreateProductRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Uuid::new_v4(),
            name: value.name,
        })
    }
}

pub async fn save_product<R: ProductRepo + ?Sized>(
    repo: &R,
    command: CreateProductCommand,
) -> anyhow::Result<Product> {
    repo.save(Product::from(command)).await
}

pub async fn get_products<R: ProductRepo + ?Sized>(repo: &R) -> anyhow::Result<Vec<Product>> {
    repo.get_all().await
}
