use serenity_additions::Error as SerenityUtilsError;
use thiserror::Error;

pub type BotResult<T> = Result<T, BotError>;

#[derive(Error, Debug)]
pub enum BotError {
    #[error("Serenity Error: {0}")]
    SerenityError(#[from] serenity::Error),

    #[error("Database Error: {0}")]
    Database(#[from] bot_database::error::DatabaseError),

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

    #[error("Reqwest Error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Detected CLI injection attempt")]
    CliInject,

    #[error("Serenity Utils Error: {0}")]
    SerenityUtils(#[from] serenity_additions::Error),

    #[error("Track Error: {0}")]
    TrackError(#[from] songbird::error::TrackError),

    #[error("JoinError: {0}")]
    JoinError(#[from] songbird::error::JoinError),

    #[error("YouTube Error: {0}")]
    YoutubeError(#[from] youtube_metadata::error::Error),

    #[error("No songbird client for current guild")]
    MissingSongbirdClient,

    #[error("{0}")]
    Msg(String),
}

impl From<&str> for BotError {
    fn from(s: &str) -> Self {
        Self::Msg(s.to_string())
    }
}

impl From<BotError> for SerenityUtilsError {
    fn from(e: BotError) -> Self {
        Self::Msg(format!("{:?}", e))
    }
}
