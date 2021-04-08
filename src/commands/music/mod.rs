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
use play::PLAY_COMMAND;
use play_next::PLAY_NEXT_COMMAND;
use queue::QUEUE_COMMAND;
use shuffle::SHUFFLE_COMMAND;
use skip::SKIP_COMMAND;

use crate::providers::music::queue::{MusicQueue, Song};
use crate::providers::music::responses::VideoInformation;
use crate::providers::music::{
    get_video_information, get_videos_for_playlist, search_video_information,
};
use crate::utils::error::{BotError, BotResult};
use crate::utils::store::Store;
use futures::future::BoxFuture;
use futures::FutureExt;
use regex::Regex;

mod clear;
mod current;
mod join;
mod leave;
mod play;
mod play_next;
mod queue;
mod shuffle;
mod skip;

#[group]
#[commands(join, leave, play, queue, skip, shuffle, current, play_next, clear)]
#[prefix("m")]
pub struct Music;

/// Joins a voice channel
async fn join_channel(ctx: &Context, channel_id: ChannelId, guild_id: GuildId) -> Arc<Mutex<Call>> {
    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let (handler, _) = manager.join(guild_id, channel_id).await;
    let mut data = ctx.data.write().await;
    let store = data.get_mut::<Store>().unwrap();
    let queue = Arc::new(Mutex::new(MusicQueue::new()));

    store.music_queues.insert(guild_id, queue.clone());
    {
        let mut handler_lock = handler.lock().await;

        handler_lock.add_global_event(
            Event::Track(TrackEvent::End),
            SongEndNotifier {
                channel_id,
                http: ctx.http.clone(),
                queue: Arc::clone(&queue),
                handler: handler.clone(),
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
async fn get_queue_for_guild(
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

struct SongEndNotifier {
    channel_id: ChannelId,
    http: Arc<Http>,
    queue: Arc<Mutex<MusicQueue>>,
    handler: Arc<Mutex<Call>>,
}

#[async_trait]
impl VoiceEventHandler for SongEndNotifier {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        play_next_in_queue(&self.http, &self.channel_id, &self.queue, &self.handler).await;

        None
    }
}

/// Plays the next song in the queue
async fn play_next_in_queue(
    http: &Arc<Http>,
    channel_id: &ChannelId,
    queue: &Arc<Mutex<MusicQueue>>,
    handler: &Arc<Mutex<Call>>,
) {
    let mut queue_lock = queue.lock().await;

    if let Some(next) = queue_lock.next() {
        let source = match songbird::ytdl(&next.url).await {
            Ok(s) => s,
            Err(e) => {
                let _ = channel_id
                    .say(&http, format!("Failed to enqueue {}: {:?}", next.title, e))
                    .await;
                return;
            }
        };
        let mut handler_lock = handler.lock().await;
        let track = handler_lock.play_only_source(source);
        queue_lock.set_current(track);
    } else {
        queue_lock.clear_current();
    }
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

    if YOUTUBE_URL_REGEX.is_match(query) {
        // try fetching the url as a playlist
        songs = get_videos_for_playlist(query)
            .await?
            .into_iter()
            .map(Song::from)
            .collect();

        // if no songs were found fetch the song as a video
        if songs.len() == 0 {
            let song: Song = get_video_information(query).await?.into();
            added_one_msg(&ctx, msg, &song).await?;
            songs.push(song);
        } else {
            added_multiple_msg(&ctx, msg, &mut songs).await?;
        }
    } else if SPOTIFY_PLAYLIST_REGEX.is_match(query) {
        // search for all songs in the playlist and search for them on youtube
        let song_names = store.spotify_api.get_songs_in_playlist(query).await?;
        songs = parallel_search_youtube(song_names).await;
        added_multiple_msg(&ctx, msg, &mut songs).await?;
    } else if SPOTIFY_ALBUM_REGEX.is_match(query) {
        // fetch all songs in the album and search for them on youtube
        let song_names = store.spotify_api.get_songs_in_album(query).await?;
        songs = parallel_search_youtube(song_names).await;
        added_multiple_msg(&ctx, msg, &mut songs).await?;
    } else if SPOTIFY_SONG_REGEX.is_match(query) {
        // fetch the song name and search it on youtube
        let name = store.spotify_api.get_song_name(query).await?;
        let song: Song = search_video_information(name.clone())
            .await?
            .ok_or(BotError::Msg(format!("Noting found for {}", name)))?
            .into();
        added_one_msg(ctx, msg, &song).await?;
        songs.push(song);
    } else {
        let song: Song = search_video_information(query.to_string())
            .await?
            .ok_or(BotError::Msg(format!("Noting found for {}", query)))?
            .into();

        added_one_msg(&ctx, msg, &song).await?;
        songs.push(song);
    }

    Ok(songs)
}

/// Searches songs on youtube in parallel
async fn parallel_search_youtube(song_names: Vec<String>) -> Vec<Song> {
    let search_futures: Vec<BoxFuture<BotResult<Option<VideoInformation>>>> = song_names
        .into_iter()
        .map(|s| search_video_information(s).boxed())
        .collect();
    let information: Vec<BotResult<Option<VideoInformation>>> =
        futures::future::join_all(search_futures).await;
    information
        .into_iter()
        .filter_map(|i| i.ok().and_then(|s| s).map(Song::from))
        .collect()
}

/// Message when one song was added to the queue
async fn added_one_msg(ctx: &Context, msg: &Message, song: &Song) -> BotResult<()> {
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|mut e| {
                e = e.description(format!("Added [{}]({}) to the queue", song.title, song.url));
                if let Some(thumb) = &song.thumbnail {
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
