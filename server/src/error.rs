#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("Product name cannot be empty or contain only white spaces")]
    ProductNameEmpty,
}
