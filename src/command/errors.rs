use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database connection error")]
    DbConnectionError(#[from] diesel::ConnectionError),
    #[error("Hashing error")]
    HashingError(#[from] bcrypt::BcryptError),
    #[error("User repository error")]
    UserRepositoryError,
    #[error("Clap error")]
    ClapError(#[from] clap::Error),
    #[error("server error")]
    Error,
}
