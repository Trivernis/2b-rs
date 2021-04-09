use std::sync::Arc;

use serenity::async_trait;
use serenity::client::Context;
use serenity::framework::standard::macros::group;
use serenity::http::Http;
use serenity::model::channel::Message;
use serenity::model::guild::Guild;
use serenity::model::id::{ChannelId, GuildId, UserId};
use songbird::{
    Call, Event, EventContext, EventHandler as VoiceEventHandler, Songbird, TrackEvent,
};
use tokio::sync::Mutex;

use clear::CLEAR_COMMAND;
use current::CURRENT_COMMAND;
use join::JOIN_COMMAND;
use leave::LEAVE_COMMAND;
use pause::PAUSE_COMMAND;
use play::PLAY_COMMAND;
use play_next::PLAY_NEXT_COMMAND;
use queue::QUEUE_COMMAND;
use shuffle::SHUFFLE_COMMAND;
use skip::SKIP_COMMAND;

use crate::providers::music::queue::{MusicQueue, Song};
use crate::providers::music::{
    get_video_information, get_videos_for_playlist, search_video_information,
};
use crate::utils::context_data::Store;
use crate::utils::error::{BotError, BotResult};
use regex::Regex;
use std::sync::atomic::{AtomicIsize, AtomicUsize, Ordering};
use std::time::Duration;

mod clear;
mod current;
mod join;
mod leave;
mod pause;
mod play;
mod play_next;
mod queue;
mod shuffle;
mod skip;

#[group]
#[commands(
    join, leave, play, queue, skip, shuffle, current, play_next, clear, pause
)]
#[prefixes("m", "music")]
pub struct Music;

struct SongEndNotifier {
    channel_id: ChannelId,
    http: Arc<Http>,
    queue: Arc<Mutex<MusicQueue>>,
    handler: Arc<Mutex<Call>>,
}

#[async_trait]
impl VoiceEventHandler for SongEndNotifier {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        log::debug!("Song ended in {}. Playing next one", self.channel_id);
        while !play_next_in_queue(&self.http, &self.channel_id, &self.queue, &self.handler).await {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        None
    }
}

struct ChannelDurationNotifier {
    channel_id: ChannelId,
    guild_id: GuildId,
    count: Arc<AtomicUsize>,
    queue: Arc<Mutex<MusicQueue>>,
    leave_in: Arc<AtomicIsize>,
    handler: Arc<Mutex<Call>>,
    manager: Arc<Songbird>,
}

#[async_trait]
impl VoiceEventHandler for ChannelDurationNotifier {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        let count_before = self.count.fetch_add(1, Ordering::Relaxed);
        log::debug!(
            "Playing in channel {} for {} minutes",
            self.channel_id,
            count_before
        );
        let queue_lock = self.queue.lock().await;
        if queue_lock.leave_flag {
            log::debug!("Waiting to leave");
            if self.leave_in.fetch_sub(1, Ordering::Relaxed) <= 0 {
                log::debug!("Leaving voice channel");
                {
                    let mut handler_lock = self.handler.lock().await;
                    handler_lock.remove_all_global_events();
                }
                if let Some(current) = queue_lock.current() {
                    let _ = current.stop();
                }
                let _ = self.manager.remove(self.guild_id).await;
                log::debug!("Left the voice channel");
            }
        } else {
            log::debug!("Resetting leave value");
            self.leave_in.store(5, Ordering::Relaxed)
        }

        None
    }
}

/// Joins a voice channel
async fn join_channel(ctx: &Context, channel_id: ChannelId, guild_id: GuildId) -> Arc<Mutex<Call>> {
    log::debug!(
        "Attempting to join channel {} in guild {}",
        channel_id,
        guild_id
    );
    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let (handler, _) = manager.join(guild_id, channel_id).await;
    let mut data = ctx.data.write().await;
    let store = data.get_mut::<Store>().unwrap();
    log::debug!("Creating new queue");
    let queue = Arc::new(Mutex::new(MusicQueue::new()));

    store.music_queues.insert(guild_id, queue.clone());
    {
        let mut handler_lock = handler.lock().await;

        log::debug!("Registering track end handler");
        handler_lock.add_global_event(
            Event::Track(TrackEvent::End),
            SongEndNotifier {
                channel_id: channel_id.clone(),
                http: ctx.http.clone(),
                queue: Arc::clone(&queue),
                handler: handler.clone(),
            },
        );

        handler_lock.add_global_event(
            Event::Periodic(Duration::from_secs(60), None),
            ChannelDurationNotifier {
                channel_id,
                guild_id,
                count: Default::default(),
                queue: Arc::clone(&queue),
                handler: handler.clone(),
                leave_in: Arc::new(AtomicIsize::new(5)),
                manager: manager.clone(),
            },
        );
    }

    handler
}

/// Returns the voice channel the author is in
fn get_channel_for_author(author_id: &UserId, guild: &Guild) -> BotResult<ChannelId> {
    guild
        .voice_states
        .get(author_id)
        .and_then(|voice_state| voice_state.channel_id)
        .ok_or(BotError::from("Not in a voice channel."))
}

/// Returns the voice manager from the context
async fn get_voice_manager(ctx: &Context) -> Arc<Songbird> {
    songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone()
}

/// Returns a reference to a guilds music queue
pub(crate) async fn get_queue_for_guild(
    ctx: &Context,
    guild_id: &GuildId,
) -> BotResult<Arc<Mutex<MusicQueue>>> {
    let data = ctx.data.read().await;
    let store = data.get::<Store>().unwrap();

    let queue = store
        .music_queues
        .get(guild_id)
        .ok_or(BotError::from("No queue for server"))?
        .clone();
    Ok(queue)
}

/// Plays the next song in the queue
async fn play_next_in_queue(
    http: &Arc<Http>,
    channel_id: &ChannelId,
    queue: &Arc<Mutex<MusicQueue>>,
    handler: &Arc<Mutex<Call>>,
) -> bool {
    let mut queue_lock = queue.lock().await;

    if let Some(mut next) = queue_lock.next() {
        let url = match next.url().await {
            Some(url) => url,
            None => {
                let _ = channel_id
                    .say(&http, format!("'{}' not found", next.title()))
                    .await;
                return false;
            }
        };
        log::debug!("Getting source for song '{}'", url);
        let source = match songbird::ytdl(&url).await {
            Ok(s) => s,
            Err(e) => {
                let _ = channel_id
                    .say(
                        &http,
                        format!("Failed to enqueue {}: {:?}", next.title(), e),
                    )
                    .await;
                return false;
            }
        };
        let mut handler_lock = handler.lock().await;
        let track = handler_lock.play_only_source(source);
        log::trace!("Track is {:?}", track);
        queue_lock.set_current(track);
    } else {
        queue_lock.clear_current();
    }
    true
}

/// Returns the list of songs for a given url
async fn get_songs_for_query(ctx: &Context, msg: &Message, query: &str) -> BotResult<Vec<Song>> {
    lazy_static::lazy_static! {
        // expressions to determine the type of url
        static ref YOUTUBE_URL_REGEX: Regex = Regex::new(r"^(https?(://))?(www\.)?(youtube\.com/watch\?.*v=.*)|(/youtu.be/.*)|(youtube\.com/playlist\?.*list=.*)$").unwrap();
        static ref SPOTIFY_PLAYLIST_REGEX: Regex = Regex::new(r"^(https?(://))?(www\.|open\.)?spotify\.com/playlist/.*").unwrap();
        static ref SPOTIFY_ALBUM_REGEX: Regex = Regex::new(r"^(https?(://))?(www\.|open\.)?spotify\.com/album/.*").unwrap();
        static ref SPOTIFY_SONG_REGEX: Regex = Regex::new(r"^(https?(://))?(www\.|open\.)?spotify\.com/track/.*").unwrap();
    }
    let mut songs = Vec::new();
    let data = ctx.data.read().await;
    let store = data.get::<Store>().unwrap();

    log::debug!("Querying play input {}", query);
    if YOUTUBE_URL_REGEX.is_match(query) {
        log::debug!("Query is youtube video or playlist");
        // try fetching the url as a playlist
        songs = get_videos_for_playlist(query)
            .await?
            .into_iter()
            .map(Song::from)
            .collect();

        // if no songs were found fetch the song as a video
        if songs.len() == 0 {
            log::debug!("Query is youtube video");
            let mut song: Song = get_video_information(query).await?.into();
            added_one_msg(&ctx, msg, &mut song).await?;
            songs.push(song);
        } else {
            log::debug!("Query is playlist with {} songs", songs.len());
            added_multiple_msg(&ctx, msg, &mut songs).await?;
        }
    } else if SPOTIFY_PLAYLIST_REGEX.is_match(query) {
        // search for all songs in the playlist and search for them on youtube
        log::debug!("Query is spotify playlist");
        songs = store.spotify_api.get_songs_in_playlist(query).await?;
        added_multiple_msg(&ctx, msg, &mut songs).await?;
    } else if SPOTIFY_ALBUM_REGEX.is_match(query) {
        // fetch all songs in the album and search for them on youtube
        log::debug!("Query is spotify album");
        songs = store.spotify_api.get_songs_in_album(query).await?;
        added_multiple_msg(&ctx, msg, &mut songs).await?;
    } else if SPOTIFY_SONG_REGEX.is_match(query) {
        // fetch the song name and search it on youtube
        log::debug!("Query is a spotify song");
        let mut song = store.spotify_api.get_song_name(query).await?;
        added_one_msg(ctx, msg, &mut song).await?;
        songs.push(song);
    } else {
        log::debug!("Query is a youtube search");
        let mut song: Song = search_video_information(query.to_string())
            .await?
            .ok_or(BotError::Msg(format!("Noting found for {}", query)))?
            .into();
        log::trace!("Search result is {:?}", song);

        added_one_msg(&ctx, msg, &mut song).await?;
        songs.push(song);
    }

    Ok(songs)
}

/// Message when one song was added to the queue
async fn added_one_msg(ctx: &Context, msg: &Message, song: &mut Song) -> BotResult<()> {
    let url = song.url().await.ok_or(BotError::from("Song not found"))?;
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|mut e| {
                e = e.description(format!("Added [{}]({}) to the queue", song.title(), url));
                if let Some(thumb) = &song.thumbnail() {
                    e = e.thumbnail(thumb);
                }

                e
            })
        })
        .await?;
    Ok(())
}

/// Message when multiple songs were added to the queue
async fn added_multiple_msg(ctx: &Context, msg: &Message, songs: &mut Vec<Song>) -> BotResult<()> {
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| e.description(format!("Added {} songs to the queue", songs.len())))
        })
        .await?;
    Ok(())
}
