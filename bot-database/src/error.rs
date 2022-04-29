use thiserror::Error;

pub type DatabaseResult<T> = Result<T, DatabaseError>;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("DotEnv Error: {0}")]
    DotEnv(#[from] dotenv::Error),

    #[error("{0}")]
    SeaOrm(#[from] sea_orm::error::DbErr),

    #[error("{0}")]
    Msg(String),
}

impl From<&str> for DatabaseError {
    fn from(s: &str) -> Self {
        Self::Msg(s.to_string())
    }
}
