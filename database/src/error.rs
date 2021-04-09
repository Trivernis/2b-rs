use thiserror::Error;

pub type DatabaseResult<T> = Result<T, DatabaseError>;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("DotEnv Error: {0}")]
    DotEnv(#[from] dotenv::Error),

    #[error("Connection Error: {0}")]
    ConnectionError(#[from] diesel::prelude::ConnectionError),

    #[error("Pool Connection Error: {0}")]
    PoolConnectionError(#[from] r2d2::Error),

    #[error("Migration Error: {0}")]
    MigrationError(#[from] diesel_migrations::RunMigrationsError),

    #[error("Result Error: {0}")]
    ResultError(#[from] diesel::result::Error),
}