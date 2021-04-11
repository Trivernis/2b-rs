use thiserror::Error;

pub type SerenityUtilsResult<T> = Result<T, SerenityUtilsError>;

#[derive(Debug, Error)]
pub enum SerenityUtilsError {
    #[error("Serenity Error: {0}")]
    SerenityError(#[from] serenity::Error),

    #[error("Page {0} not found")]
    PageNotFound(usize),

    #[error("Serenity Utils not fully initialized")]
    Uninitialized,
}
