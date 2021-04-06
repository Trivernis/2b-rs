use thiserror::Error;

pub type BotResult<T> = Result<T, BotError>;

#[derive(Error, Debug)]
pub enum BotError {
    #[error("Serenity Error: {0}")]
    SerenityError(#[from] serenity::Error),

    #[error("Sqlite Error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("Missing Bot Token")]
    MissingToken,
}
