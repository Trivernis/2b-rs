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

use crate::providers::music::{get_video_information, get_videos_for_playlist};
use crate::providers::music::queue::{MusicQueue, Song};
use crate::utils::error::{BotError, BotResult};
use crate::utils::store::Store;

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
async fn get_songs_for_url(ctx: &&Context, msg: &Message, url: &str) -> BotResult<Vec<Song>> {
    let mut songs: Vec<Song> = get_videos_for_playlist(url)?
        .into_iter()
        .map(Song::from)
        .collect();
    if songs.len() == 0 {
        let song: Song = get_video_information(url)?.into();
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
        songs.push(song);
    } else {
        msg.channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| e.description(format!("Added {} songs to the queue", songs.len())))
            })
            .await?;
    }

    Ok(songs)
}
