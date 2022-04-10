use crate::utils::context_data::MusicPlayers;
use lavalink_rs::gateway::LavalinkEventHandler;
use lavalink_rs::model::{PlayerUpdate, Stats, TrackFinish, TrackStart};
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
        tracing::info!("Track started!\nGuild: {}", event.guild_id);
    }

    async fn track_finish(&self, _: LavalinkClient, event: TrackFinish) {
        tracing::info!("Track finished!\nGuild: {}", event.guild_id);
        let player = {
            let data = self.data.read().await;
            let players = data.get::<MusicPlayers>().unwrap();

            players.get(&event.guild_id.0).cloned()
        };
        if let Some(player) = player {
            let mut player = player.lock().await;
            if let Err(e) = player.play_next().await {
                tracing::error!("Failed to play next song: {:?}", e);
            }
            if let Err(e) = player.update_now_playing().await {
                tracing::error!("Failed to update now playing embed: {:?}", e);
            }
        }
    }

    async fn player_update(&self, _: LavalinkClient, event: PlayerUpdate) {
        tracing::debug!("Received player update event: {:?}", event);
    }

    async fn stats(&self, _: LavalinkClient, event: Stats) {
        tracing::debug!("Received stats event: {:?}", event);
    }
}

pub struct Lavalink;

impl TypeMapKey for Lavalink {
    type Value = Arc<LavalinkClient>;
}
