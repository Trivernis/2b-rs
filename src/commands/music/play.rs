use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;

use crate::commands::common::handle_autodelete;
use crate::commands::music::{
    get_channel_for_author, get_music_player_for_guild, get_songs_for_query,
};
use crate::messages::music::now_playing::create_now_playing_msg;
use crate::providers::music::player::MusicPlayer;
use crate::providers::settings::{get_setting, Setting};
use std::sync::Arc;

#[command]
#[only_in(guilds)]
#[description("Plays a song in a voice channel")]
#[usage("(<spotify_ur>|<youtube_url>|<query>|pl:<saved_playlist>)")]
#[min_args(1)]
#[aliases("p")]
#[bucket("music_api")]
async fn play(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let query = args.message();

    let guild = msg.guild(&ctx.cache).unwrap();
    tracing::debug!("Play request received for guild {}", guild.id);

    let mut player = get_music_player_for_guild(ctx, guild.id).await;

    if player.is_none() {
        tracing::debug!("Not in a channel. Joining authors channel...");
        let channel_id = get_channel_for_author(&msg.author.id, &guild)?;
        let music_player = MusicPlayer::join(ctx, guild.id, channel_id, msg.channel_id).await?;
        player = Some(music_player);
    }
    let player = player.unwrap();
    let songs = get_songs_for_query(&ctx, msg, query).await?;

    let (play_first, create_now_playing) = {
        tracing::debug!("Adding song to queue");
        let mut player_lock = player.lock().await;
        for song in songs {
            player_lock.queue().add(song);
        }
        let autoshuffle = get_setting(ctx, guild.id, Setting::MusicAutoShuffle)
            .await?
            .unwrap_or(false);

        if autoshuffle {
            tracing::debug!("Autoshuffeling");
            player_lock.queue().shuffle();
        }
        (
            player_lock.queue().current().is_none(),
            player_lock.now_playing_message().is_none(),
        )
    };

    if play_first {
        tracing::debug!("Playing first song in queue");
        let mut player_lock = player.lock().await;
        player_lock.play_next().await?;
    }
    if create_now_playing {
        let handle = create_now_playing_msg(ctx, Arc::clone(&player), msg.channel_id).await?;
        let mut player_lock = player.lock().await;
        player_lock.set_now_playing(handle).await;
    }
    handle_autodelete(ctx, msg).await?;

    Ok(())
}
