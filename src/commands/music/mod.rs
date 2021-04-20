use std::sync::Arc;

use aspotify::Track;
use regex::Regex;
use serenity::client::Context;
use serenity::framework::standard::macros::{check, group};
use serenity::model::channel::Message;
use serenity::model::guild::Guild;
use serenity::model::id::{ChannelId, GuildId, UserId};
use serenity::model::user::User;
use songbird::Songbird;
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

use crate::providers::music::player::MusicPlayer;
use crate::providers::music::queue::Song;
use crate::providers::music::{add_youtube_song_to_database, youtube_dl};
use crate::providers::settings::{get_setting, Setting};
use crate::utils::context_data::{DatabaseContainer, MusicPlayers, Store};
use crate::utils::error::{BotError, BotResult};
use bot_database::Database;
use futures::future::BoxFuture;
use futures::FutureExt;
use serenity::framework::standard::{Args, CommandOptions, Reason};
use youtube_metadata::get_video_information;

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

/// Returns the voice manager from the context
pub async fn get_voice_manager(ctx: &Context) -> Arc<Songbird> {
    songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone()
}

/// Returns the voice channel the author is in
fn get_channel_for_author(author_id: &UserId, guild: &Guild) -> BotResult<ChannelId> {
    guild
        .voice_states
        .get(author_id)
        .and_then(|voice_state| voice_state.channel_id)
        .ok_or(BotError::from("You're not in a Voice Channel"))
}

/// Returns the music player for a given guild
pub async fn get_music_player_for_guild(
    ctx: &Context,
    guild_id: GuildId,
) -> Option<Arc<Mutex<MusicPlayer>>> {
    let data = ctx.data.read().await;
    let players = data.get::<MusicPlayers>().unwrap();

    players.get(&guild_id.0).cloned()
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
            let mut song: Song = get_video_information(&query).await?.into();
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

        let futures: Vec<BoxFuture<Song>> = tracks
            .into_iter()
            .map(|track| {
                async {
                    get_youtube_song_for_track(&database, track.clone())
                        .await
                        .unwrap_or(None)
                        .unwrap_or(track.into())
                }
                .boxed()
            })
            .collect();
        songs = futures::future::join_all(futures).await;

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

#[check]
#[name = "DJ"]
pub async fn check_dj(
    ctx: &Context,
    msg: &Message,
    _: &mut Args,
    _: &CommandOptions,
) -> Result<(), Reason> {
    let guild = msg
        .guild(&ctx.cache)
        .await
        .ok_or(Reason::Log("Not in a guild".to_string()))?;

    if is_dj(ctx, guild.id, &msg.author)
        .await
        .map_err(|e| Reason::Log(format!("{:?}", e)))?
    {
        Ok(())
    } else {
        Err(Reason::User("Lacking DJ role".to_string()))
    }
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

        if let Some(song) = entry {
            // check if the video is still available
            log::trace!("Found entry is {:?}", song);
            if let Ok(info) = get_video_information(&song.url).await {
                return Ok(Some(info.into()));
            } else {
                log::debug!("Video '{}' is not available. Deleting entry", song.url);
                database.delete_song(song.id).await?;
                return Ok(None);
            }
        }
        Ok(None)
    } else {
        log::debug!("Track has no ID");
        Ok(None)
    }
}
