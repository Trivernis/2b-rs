pub use gifs::*;
pub use guild_playlists::*;
pub use guild_playlists::*;
pub use statistics::*;

use crate::PoolConnection;

mod gifs;
mod guild_playlists;
mod guild_settings;
mod statistics;

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
