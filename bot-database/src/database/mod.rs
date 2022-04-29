pub use ephemeral_messages::*;
pub use guild_playlists::*;
pub use guild_playlists::*;
pub use media::*;
use sea_orm::DatabaseConnection;
pub use statistics::*;
pub use youtube_songs::*;

mod ephemeral_messages;
mod guild_playlists;
mod guild_settings;
mod media;
mod statistics;
mod youtube_songs;

#[derive(Clone)]
pub struct BotDatabase {
    db: DatabaseConnection,
}

impl BotDatabase {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}
