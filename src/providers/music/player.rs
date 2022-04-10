use crate::messages::music::now_playing::update_now_playing_msg;
use crate::providers::music::lavalink::Lavalink;
use crate::providers::music::lyrics::get_lyrics;
use crate::providers::music::queue::MusicQueue;
use crate::utils::context_data::MusicPlayers;
use crate::utils::error::{BotError, BotResult};
use lavalink_rs::LavalinkClient;
use serenity::prelude::TypeMap;
use serenity::{
    client::Context,
    http::Http,
    model::id::{ChannelId, GuildId},
};
use serenity_rich_interaction::core::{MessageHandle, SHORT_TIMEOUT};
use serenity_rich_interaction::ephemeral_message::EphemeralMessage;
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
    msg_channel: ChannelId,
    leave_flag: bool,
    paused: bool,
    equalizer: [f64; 15],
}

impl MusicPlayer {
    /// Creates a new music player
    pub fn new(
        client: Arc<LavalinkClient>,
        http: Arc<Http>,
        guild_id: GuildId,
        msg_channel: ChannelId,
    ) -> Self {
        Self {
            client,
            http,
            guild_id,
            queue: MusicQueue::new(),
            msg_channel,
            now_playing_msg: None,
            leave_flag: false,
            paused: false,
            equalizer: [0f64; 15],
        }
    }

    /// Joins a given voice channel
    pub async fn join(
        ctx: &Context,
        guild_id: GuildId,
        voice_channel_id: ChannelId,
        msg_channel_id: ChannelId,
    ) -> BotResult<Arc<Mutex<MusicPlayer>>> {
        let manager = songbird::get(ctx).await.unwrap();
        let (handler, connection) = manager.join_gateway(guild_id, voice_channel_id).await;
        let connection = connection?;

        {
            let mut handler = handler.lock().await;
            handler.deafen(true).await?;
        }

        let player = {
            let mut data = ctx.data.write().await;
            let client = data.get::<Lavalink>().unwrap();
            client.create_session_with_songbird(&connection).await?;
            let player = MusicPlayer::new(
                Arc::clone(client),
                Arc::clone(&ctx.http),
                guild_id,
                msg_channel_id,
            );
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
        if self.paused {
            self.client.pause(self.guild_id).await?;
        }

        Ok(())
    }

    /// Tries to play the next song
    pub async fn try_play_next(&mut self) -> BotResult<bool> {
        let mut next = if let Some(n) = self.queue.next() {
            tracing::trace!("Next is {:?}", n);
            n
        } else {
            return Ok(true);
        };
        let url = if let Some(url) = next.url().await {
            url
        } else {
            self.send_error_message(format!(
                "‼️ Could not find a video to play for '{}' by '{}'",
                next.title(),
                next.author()
            ))
            .await?;
            tracing::debug!("Could not find playable candidate for song.");
            return Ok(false);
        };
        let query_information = match self.client.auto_search_tracks(url).await {
            Ok(i) => i,
            Err(e) => {
                tracing::error!("Failed to search for song: {}", e);
                self.send_error_message(format!(
                    "‼️ Failed to retrieve information for song '{}' by '{}': {:?}",
                    next.title(),
                    next.author(),
                    e
                ))
                .await?;
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

    /// Sends a play error message to the players test channel
    async fn send_error_message(&self, content: String) -> BotResult<()> {
        EphemeralMessage::create(&self.http, self.msg_channel, SHORT_TIMEOUT, |m| {
            m.content(content)
        })
        .await?;

        Ok(())
    }

    /// Returns the equalizer
    pub fn get_equalizer(&self) -> &[f64; 15] {
        &self.equalizer
    }

    /// Equalizes a specified band
    pub async fn equalize(&mut self, band: u8, value: f64) -> BotResult<()> {
        if band > 15 {
            return Err(BotError::from("Invalid Equalizer band"));
        }
        if value < -0.25 || value > 0.25 {
            return Err(BotError::from("Invalid Equalizer value"));
        }
        self.equalizer[band as usize] = value;
        self.client
            .equalize_all(self.guild_id, self.equalizer)
            .await?;

        Ok(())
    }

    /// Equalizes all bands at the same time
    pub async fn equalize_all(&mut self, bands: [f64; 15]) -> BotResult<()> {
        self.equalizer = bands;
        self.client
            .equalize_all(self.guild_id, self.equalizer)
            .await?;

        Ok(())
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
                tracing::debug!("Waiting to leave");

                if leave_in <= 0 {
                    tracing::debug!("Leaving voice channel");

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
                    tracing::debug!("Left the voice channel");
                    return;
                }
                leave_in -= 1;
            } else {
                tracing::debug!("Resetting leave value");
                leave_in = 5
            }
        }
    });
}
