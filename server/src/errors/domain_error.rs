#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("Product name is empty")]
    ProductNameEmpty,
}
