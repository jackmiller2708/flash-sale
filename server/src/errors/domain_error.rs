#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    // Product domain
    #[error("product name cannot be empty")]
    ProductNameEmpty,

    #[error("product price must be positive")]
    ProductPriceInvalid,

    // User domain
    #[error("invalid email format: {0}")]
    InvalidEmail(String),

    #[error("username must be between 3 and 50 characters")]
    InvalidUsername,

    // Flash sale domain
    #[error("flash sale start time must be in the future")]
    InvalidFlashSaleStartTime,

    #[error("flash sale end time must be after start time")]
    InvalidFlashSaleEndTime,

    #[error("flash sale quantity must be positive")]
    InvalidFlashSaleQuantity,

    // Order domain
    #[error("order quantity must be positive")]
    InvalidOrderQuantity,

    #[error("cannot modify completed order")]
    OrderAlreadyCompleted,
}
