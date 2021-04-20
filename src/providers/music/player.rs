use crate::messages::music::now_playing::update_now_playing_msg;
use crate::providers::music::lavalink::Lavalink;
use crate::providers::music::lyrics::get_lyrics;
use crate::providers::music::queue::MusicQueue;
use crate::utils::context_data::MusicPlayers;
use crate::utils::error::BotResult;
use bot_serenityutils::core::MessageHandle;
use lavalink_rs::LavalinkClient;
use serenity::prelude::TypeMap;
use serenity::{
    client::Context,
    http::Http,
    model::id::{ChannelId, GuildId},
};
use songbird::Songbird;
use std::mem;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Mutex, RwLock};

pub struct MusicPlayer {
    client: Arc<LavalinkClient>,
    http: Arc<Http>,
    queue: MusicQueue,
    guild_id: GuildId,
    now_playing_msg: Option<Arc<RwLock<MessageHandle>>>,
    leave_flag: bool,
    paused: bool,
}

impl MusicPlayer {
    /// Creates a new music player
    pub fn new(client: Arc<LavalinkClient>, http: Arc<Http>, guild_id: GuildId) -> Self {
        Self {
            client,
            http,
            guild_id,
            queue: MusicQueue::new(),
            now_playing_msg: None,
            leave_flag: false,
            paused: false,
        }
    }

    /// Joins a given voice channel
    pub async fn join(
        ctx: &Context,
        guild_id: GuildId,
        voice_channel_id: ChannelId,
    ) -> BotResult<Arc<Mutex<MusicPlayer>>> {
        let manager = songbird::get(ctx).await.unwrap();
        let (_, connection) = manager.join_gateway(guild_id, voice_channel_id).await;
        let connection = connection?;

        let player = {
            let mut data = ctx.data.write().await;
            let client = data.get::<Lavalink>().unwrap();
            client.create_session(&connection).await?;
            let player = MusicPlayer::new(Arc::clone(client), Arc::clone(&ctx.http), guild_id);
            let player = Arc::new(Mutex::new(player));
            let players = data.get_mut::<MusicPlayers>().unwrap();
            players.insert(guild_id.0, Arc::clone(&player));
            player
        };

        wait_for_disconnect(
            Arc::clone(&ctx.data),
            Arc::clone(&player),
            manager,
            guild_id,
        );

        Ok(player)
    }

    /// Returns a mutable reference to the inner queue
    pub fn queue(&mut self) -> &mut MusicQueue {
        &mut self.queue
    }

    /// Skips to the next song
    pub async fn skip(&mut self) -> BotResult<()> {
        self.client.stop(self.guild_id.0).await?;

        Ok(())
    }

    /// Stops playback and leaves the channel
    pub async fn stop(&mut self) -> BotResult<()> {
        self.queue.clear();
        self.client.stop(self.guild_id.0).await?;
        Ok(())
    }

    /// Returns the lyrics for the currently playing song
    pub async fn lyrics(&self) -> BotResult<Option<String>> {
        if let Some(current) = self.queue.current() {
            let title = current.title();
            let artist = current.author();
            get_lyrics(artist, title).await
        } else {
            Ok(None)
        }
    }

    /// Plays the next song in the queue
    pub async fn play_next(&mut self) -> BotResult<()> {
        while !self.try_play_next().await? {}

        Ok(())
    }

    /// Tries to play the next song
    pub async fn try_play_next(&mut self) -> BotResult<bool> {
        let mut next = if let Some(n) = self.queue.next() {
            n
        } else {
            return Ok(true);
        };
        let url = if let Some(url) = next.url().await {
            url
        } else {
            return Ok(false);
        };
        let query_information = match self.client.auto_search_tracks(url).await {
            Ok(i) => i,
            Err(e) => {
                log::error!("Failed to search for song: {}", e);
                return Ok(false);
            }
        };

        if query_information.tracks.len() == 0 {
            return Ok(false);
        }
        let track = query_information.tracks[0].clone();
        self.client.play(self.guild_id.0, track).start().await?;
        self.queue.set_current(next);

        Ok(true)
    }

    /// Sets the new now playing message of the queue
    pub async fn set_now_playing(&mut self, message: Arc<RwLock<MessageHandle>>) {
        let _ = self.delete_now_playing().await;
        self.now_playing_msg = Some(message)
    }

    /// Updates the now playing message
    pub async fn update_now_playing(&self) -> BotResult<()> {
        if let (Some(current), Some(np)) = (self.queue.current(), &self.now_playing_msg) {
            update_now_playing_msg(&self.http, np, &mut current.clone(), self.is_paused()).await?;
        }

        Ok(())
    }

    /// Deletes the now playing message
    pub async fn delete_now_playing(&mut self) -> BotResult<()> {
        if let Some(np) = mem::take(&mut self.now_playing_msg) {
            let np = np.read().await;
            let msg = np.get_message(&self.http).await?;
            msg.delete(&self.http).await?;
        }

        Ok(())
    }

    /// Pauses playback
    pub async fn toggle_paused(&mut self) -> BotResult<()> {
        self.paused = !self.paused;
        self.client.set_pause(self.guild_id.0, self.paused).await?;

        Ok(())
    }

    /// Returns if playback is paused
    pub fn is_paused(&self) -> bool {
        self.paused
    }

    /// Returns the now playing message of the player
    pub fn now_playing_message(&self) -> &Option<Arc<RwLock<MessageHandle>>> {
        &self.now_playing_msg
    }

    /// Deletes the now playing message from the player
    pub fn clear_now_playing(&mut self) {
        self.now_playing_msg = None;
    }

    /// Sets the leave flag to the given value
    pub fn set_leave_flag(&mut self, flag: bool) {
        self.leave_flag = flag;
    }
}

/// Stats a tokio coroutine to check for player disconnect conditions
fn wait_for_disconnect(
    data: Arc<RwLock<TypeMap>>,
    player: Arc<Mutex<MusicPlayer>>,
    manager: Arc<Songbird>,
    guild_id: GuildId,
) {
    let mut leave_in: i32 = 5;
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(60)).await;
            if manager.get(guild_id).is_none() {
                return; // leave when there's no connection to handle
            }
            let mut player_lock = player.lock().await;

            if player_lock.leave_flag {
                log::debug!("Waiting to leave");

                if leave_in <= 0 {
                    log::debug!("Leaving voice channel");

                    if let Some(handler) = manager.get(guild_id) {
                        let mut handler_lock = handler.lock().await;
                        let _ = handler_lock.leave().await;
                    }

                    let _ = manager.remove(guild_id).await;
                    let mut data = data.write().await;
                    let players = data.get_mut::<MusicPlayers>().unwrap();
                    players.remove(&guild_id.0);
                    let _ = player_lock.stop().await;
                    let _ = player_lock.delete_now_playing().await;
                    log::debug!("Left the voice channel");
                    return;
                }
                leave_in -= 1;
            } else {
                log::debug!("Resetting leave value");
                leave_in = 5
            }
        }
    });
}
