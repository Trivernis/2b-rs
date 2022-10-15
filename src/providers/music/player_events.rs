use serenity::async_trait;
use std::sync::Arc;

use serenity::prelude::Mutex;
use songbird::{Call, Event, EventContext, EventHandler, TrackEvent};

use super::player::MusicPlayer;

pub fn register_player_events(player: Arc<Mutex<MusicPlayer>>, handler: &mut Call) {
    handler.add_global_event(Event::Track(TrackEvent::End), TrackEndHandler { player });
}

struct TrackEndHandler {
    player: Arc<Mutex<MusicPlayer>>,
}

#[async_trait]
impl EventHandler for TrackEndHandler {
    #[tracing::instrument(level = "debug", skip_all)]
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        let mut player = self.player.lock().await;
        if let Err(e) = player.play_next().await {
            tracing::error!("Failed to play next song: {:?}", e);
        }
        if let Err(e) = player.update_now_playing().await {
            tracing::error!("Failed to update now playing embed: {:?}", e);
        }
        None
    }
}
