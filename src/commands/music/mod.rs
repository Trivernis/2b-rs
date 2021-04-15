use std::mem;
use std::sync::atomic::{AtomicIsize, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

use regex::Regex;
use serenity::async_trait;
use serenity::client::Context;
use serenity::framework::standard::macros::group;
use serenity::http::Http;
use serenity::model::channel::Message;
use serenity::model::guild::Guild;
use serenity::model::id::{ChannelId, GuildId, UserId};
use serenity::model::user::User;
use songbird::{
    Call, Event, EventContext, EventHandler as VoiceEventHandler, Songbird, TrackEvent,
};
use tokio::sync::Mutex;

use clear_queue::CLEAR_QUEUE_COMMAND;
use current::CURRENT_COMMAND;
use join::JOIN_COMMAND;
use leave::LEAVE_COMMAND;
use lyrics::LYRICS_COMMAND;
use move_song::MOVE_SONG_COMMAND;
use pause::PAUSE_COMMAND;
use play::PLAY_COMMAND;
use play_next::PLAY_NEXT_COMMAND;
use playlists::PLAYLISTS_COMMAND;
use queue::QUEUE_COMMAND;
use remove_song::REMOVE_SONG_COMMAND;
use save_playlist::SAVE_PLAYLIST_COMMAND;
use shuffle::SHUFFLE_COMMAND;
use skip::SKIP_COMMAND;

use crate::messages::music::now_playing::update_now_playing_msg;
use crate::providers::music::queue::{MusicQueue, Song};
use crate::providers::music::{add_youtube_song_to_database, youtube_dl};
use crate::providers::settings::{get_setting, Setting};
use crate::utils::context_data::{DatabaseContainer, Store};
use crate::utils::error::{BotError, BotResult};
use aspotify::Track;
use bot_database::Database;

mod clear_queue;
mod current;
mod join;
mod leave;
mod lyrics;
mod move_song;
mod pause;
mod play;
mod play_next;
mod playlists;
mod queue;
mod remove_song;
mod save_playlist;
mod shuffle;
mod skip;

#[group]
#[commands(
    join,
    leave,
    play,
    queue,
    skip,
    shuffle,
    current,
    play_next,
    clear_queue,
    pause,
    save_playlist,
    playlists,
    lyrics,
    move_song,
    remove_song
)]
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
                if let Some((current, _)) = queue_lock.current() {
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
        .ok_or(BotError::from("You're not in a Voice Channel"))
}

/// Returns the voice manager from the context
pub async fn get_voice_manager(ctx: &Context) -> Arc<Songbird> {
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
        .ok_or(BotError::from("I'm not in a Voice Channel"))?
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

        if queue_lock.paused() {
            let _ = track.pause();
        }

        if let Some(np) = &queue_lock.now_playing_msg {
            if let Err(e) =
                update_now_playing_msg(http, np, track.metadata(), queue_lock.paused()).await
            {
                log::error!("Failed to update now playing message: {:?}", e);
            }
        }
        queue_lock.set_current(track, next);
    } else {
        if let Some(np) = mem::take(&mut queue_lock.now_playing_msg) {
            let np = np.read().await;
            if let Ok(message) = np.get_message(http).await {
                let _ = message.delete(http).await;
            }
        }
        queue_lock.clear_current();
    }
    true
}

/// Returns the list of songs for a given url
async fn get_songs_for_query(ctx: &Context, msg: &Message, query: &str) -> BotResult<Vec<Song>> {
    let guild_id = msg.guild_id.unwrap();
    let mut query = query.to_string();
    lazy_static::lazy_static! {
        // expressions to determine the type of url
        static ref PLAYLIST_NAME_REGEX: Regex = Regex::new(r"^pl:(\S+)$").unwrap();
        static ref YOUTUBE_URL_REGEX: Regex = Regex::new(r"^(https?(://))?(www\.)?(youtube\.com/watch\?.*v=.*)|(/youtu.be/.*)|(youtube\.com/playlist\?.*list=.*)$").unwrap();
        static ref SPOTIFY_PLAYLIST_REGEX: Regex = Regex::new(r"^(https?(://))?(www\.|open\.)?spotify\.com/playlist/.*").unwrap();
        static ref SPOTIFY_ALBUM_REGEX: Regex = Regex::new(r"^(https?(://))?(www\.|open\.)?spotify\.com/album/.*").unwrap();
        static ref SPOTIFY_SONG_REGEX: Regex = Regex::new(r"^(https?(://))?(www\.|open\.)?spotify\.com/track/.*").unwrap();
    }
    let mut songs = Vec::new();
    let data = ctx.data.read().await;
    let store = data.get::<Store>().unwrap();
    let database = data.get::<DatabaseContainer>().unwrap();

    log::debug!("Querying play input {}", query);
    if let Some(captures) = PLAYLIST_NAME_REGEX.captures(&query) {
        log::debug!("Query is a saved playlist");
        let pl_name: &str = captures.get(1).unwrap().as_str();
        log::trace!("Playlist name is {}", pl_name);
        let playlist_opt = database
            .get_guild_playlist(guild_id.0, pl_name.to_string())
            .await?;
        log::trace!("Playlist is {:?}", playlist_opt);

        if let Some(playlist) = playlist_opt {
            log::debug!("Assigning url for saved playlist to query");
            query = playlist.url;
        }
    }
    if YOUTUBE_URL_REGEX.is_match(&query) {
        log::debug!("Query is youtube video or playlist");
        // try fetching the url as a playlist
        songs = youtube_dl::get_videos_for_playlist(&query)
            .await?
            .into_iter()
            .map(Song::from)
            .collect();

        // if no songs were found fetch the song as a video
        if songs.len() == 0 {
            log::debug!("Query is youtube video");
            let mut song: Song = youtube_dl::get_video_information(&query).await?.into();
            added_one_msg(&ctx, msg, &mut song).await?;
            add_youtube_song_to_database(&store, &database, &mut song).await?;
            songs.push(song);
        } else {
            log::debug!("Query is playlist with {} songs", songs.len());
            added_multiple_msg(&ctx, msg, &mut songs).await?;
        }
    } else if SPOTIFY_PLAYLIST_REGEX.is_match(&query) {
        // search for all songs in the playlist and search for them on youtube
        log::debug!("Query is spotify playlist");
        let tracks = store.spotify_api.get_songs_in_playlist(&query).await?;

        for track in tracks {
            songs.push(
                get_youtube_song_for_track(&database, track.clone())
                    .await?
                    .unwrap_or(track.into()),
            )
        }

        added_multiple_msg(&ctx, msg, &mut songs).await?;
    } else if SPOTIFY_ALBUM_REGEX.is_match(&query) {
        // fetch all songs in the album and search for them on youtube
        log::debug!("Query is spotify album");
        let tracks = store.spotify_api.get_songs_in_album(&query).await?;

        for track in tracks {
            songs.push(
                get_youtube_song_for_track(&database, track.clone())
                    .await?
                    .unwrap_or(track.into()),
            )
        }

        added_multiple_msg(&ctx, msg, &mut songs).await?;
    } else if SPOTIFY_SONG_REGEX.is_match(&query) {
        // fetch the song name and search it on youtube
        log::debug!("Query is a spotify song");
        let track = store.spotify_api.get_track_for_url(&query).await?;
        let mut song = get_youtube_song_for_track(&database, track.clone())
            .await?
            .unwrap_or(track.into());
        added_one_msg(ctx, msg, &mut song).await?;
        songs.push(song);
    } else {
        log::debug!("Query is a youtube search");
        let mut song: Song = youtube_dl::search_video_information(query.clone())
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

/// Returns if the given user is a dj in the given guild based on the
/// setting for the name of the dj role
pub async fn is_dj(ctx: &Context, guild: GuildId, user: &User) -> BotResult<bool> {
    let dj_role = get_setting::<String>(ctx, guild, Setting::MusicDjRole).await?;

    if let Some(role_name) = dj_role {
        let roles = ctx.http.get_guild_roles(guild.0).await?;
        let role_result = roles.iter().find(|r| r.name == role_name);

        if let Some(role) = role_result {
            Ok(user.has_role(ctx, guild, role.id).await?)
        } else {
            Ok(false)
        }
    } else {
        Ok(true)
    }
}

/// Searches for a matching youtube song for the given track in the local database
async fn get_youtube_song_for_track(database: &Database, track: Track) -> BotResult<Option<Song>> {
    log::debug!("Trying to find track in database.");
    if let Some(id) = track.id {
        let entry = database.get_song(&id).await?;
        log::trace!("Found entry is {:?}", entry);
        Ok(entry.map(Song::from))
    } else {
        log::debug!("Track has no ID");
        Ok(None)
    }
}
