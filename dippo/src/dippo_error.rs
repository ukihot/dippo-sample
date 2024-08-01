use thiserror::Error;

#[derive(Debug, Error)]
pub enum StockpileError {
    #[error("The service is already registered.")]
    AlreadyRegistered,
}

#[derive(Debug, Error)]
pub enum SpitUpError {
    #[error("The requested service was not found.")]
    NotFound,
}
