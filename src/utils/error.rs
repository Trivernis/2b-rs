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

    #[error("Minecraft Data Error: {0}")]
    MinecraftDataError(#[from] minecraft_data_rs::DataError),

    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("JSON Error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Spotify API Error: {0}")]
    SpotifyError(#[from] aspotify::Error),

    #[error("{0}")]
    Msg(String),
}

impl From<&str> for BotError {
    fn from(s: &str) -> Self {
        Self::Msg(s.to_string())
    }
}
