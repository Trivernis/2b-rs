use crate::utils::context_data::MusicPlayers;
use lavalink_rs::gateway::LavalinkEventHandler;
use lavalink_rs::model::{TrackFinish, TrackStart};
use lavalink_rs::LavalinkClient;
use serenity::async_trait;
use serenity::prelude::TypeMapKey;
use std::sync::Arc;
use tokio::sync::RwLock;
use typemap_rev::TypeMap;

pub struct LavalinkHandler {
    pub data: Arc<RwLock<TypeMap>>,
}

#[async_trait]
impl LavalinkEventHandler for LavalinkHandler {
    async fn track_start(&self, _client: LavalinkClient, event: TrackStart) {
        log::info!("Track started!\nGuild: {}", event.guild_id);
    }
    async fn track_finish(&self, _: LavalinkClient, event: TrackFinish) {
        log::info!("Track finished!\nGuild: {}", event.guild_id);
        let player = {
            let data = self.data.read().await;
            let players = data.get::<MusicPlayers>().unwrap();

            players.get(&event.guild_id).cloned()
        };
        if let Some(player) = player {
            let mut player = player.lock().await;
            if let Err(e) = player.play_next().await {
                log::error!("Failed to play next song: {:?}", e);
            }
            if let Err(e) = player.update_now_playing().await {
                log::error!("Failed to update now playing embed: {:?}", e);
            }
        }
    }
}

pub struct Lavalink;

impl TypeMapKey for Lavalink {
    type Value = Arc<LavalinkClient>;
}
