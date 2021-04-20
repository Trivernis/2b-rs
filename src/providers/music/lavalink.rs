use crate::commands::music::play_next_in_queue;
use crate::utils::context_data::Store;
use lavalink_rs::gateway::LavalinkEventHandler;
use lavalink_rs::model::{TrackFinish, TrackStart};
use lavalink_rs::LavalinkClient;
use serenity::async_trait;
use serenity::http::Http;
use serenity::model::id::GuildId;
use serenity::prelude::TypeMapKey;
use std::sync::Arc;
use tokio::sync::RwLock;
use typemap_rev::TypeMap;

pub struct LavalinkHandler {
    pub data: Arc<RwLock<TypeMap>>,
    pub http: Arc<Http>,
}

#[async_trait]
impl LavalinkEventHandler for LavalinkHandler {
    async fn track_start(&self, _client: LavalinkClient, event: TrackStart) {
        log::info!("Track started!\nGuild: {}", event.guild_id);
    }
    async fn track_finish(&self, client: LavalinkClient, event: TrackFinish) {
        log::info!("Track finished!\nGuild: {}", event.guild_id);
        let queue = {
            let data = self.data.read().await;
            let store = data.get::<Store>().unwrap();

            store
                .music_queues
                .get(&GuildId(event.guild_id))
                .unwrap()
                .clone()
        };
        let channel_id = {
            let queue = queue.lock().await;
            queue.channel_id()
        };
        while !play_next_in_queue(
            &self.http,
            &channel_id,
            &GuildId(event.guild_id),
            &queue,
            &client,
        )
        .await
        {}
    }
}

pub struct Lavalink;

impl TypeMapKey for Lavalink {
    type Value = LavalinkClient;
}
