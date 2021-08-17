pub use ephemeral_messages::*;
pub use guild_playlists::*;
pub use guild_playlists::*;
pub use media::*;
pub use statistics::*;
pub use youtube_songs::*;

use crate::PoolConnection;

mod ephemeral_messages;
mod events;
mod guild_playlists;
mod guild_settings;
mod media;
mod statistics;
mod youtube_songs;

#[derive(Clone)]
pub struct Database {
    pool: PoolConnection,
}

unsafe impl Send for Database {}

unsafe impl Sync for Database {}

impl Database {
    pub fn new(pool: PoolConnection) -> Self {
        Self { pool }
    }
}
