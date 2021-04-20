use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;

use crate::commands::common::handle_autodelete;
use crate::commands::music::{
    get_channel_for_author, get_queue_for_guild, get_songs_for_query, get_voice_manager,
    join_channel, play_next_in_queue,
};
use crate::messages::music::now_playing::create_now_playing_msg;
use crate::providers::music::lavalink::Lavalink;
use crate::providers::settings::{get_setting, Setting};

#[command]
#[only_in(guilds)]
#[description("Plays a song in a voice channel")]
#[usage("(<spotify_ur>|<youtube_url>|<query>|pl:<saved_playlist>)")]
#[min_args(1)]
#[aliases("p")]
#[bucket("music_api")]
async fn play(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let query = args.message();

    let guild = msg.guild(&ctx.cache).await.unwrap();
    log::debug!("Play request received for guild {}", guild.id);

    let manager = get_voice_manager(ctx).await;
    let handler = manager.get(guild.id);

    if handler.is_none() {
        log::debug!("Not in a channel. Joining authors channel...");
        msg.guild(&ctx.cache).await.unwrap();
        let channel_id = get_channel_for_author(&msg.author.id, &guild)?;
        join_channel(ctx, channel_id, guild.id).await;
    }

    let songs = get_songs_for_query(&ctx, msg, query).await?;

    let queue = get_queue_for_guild(ctx, &guild.id).await?;

    let (play_first, create_now_playing) = {
        log::debug!("Adding song to queue");
        let mut queue_lock = queue.lock().await;
        for song in songs {
            queue_lock.add(song);
        }
        let autoshuffle = get_setting(ctx, guild.id, Setting::MusicAutoShuffle)
            .await?
            .unwrap_or(false);

        if autoshuffle {
            log::debug!("Autoshuffeling");
            queue_lock.shuffle();
        }
        (
            queue_lock.current().is_none(),
            queue_lock.now_playing_msg.is_none(),
        )
    };

    if play_first {
        log::debug!("Playing first song in queue");
        let data = ctx.data.read().await;
        let lava_player = data.get::<Lavalink>().unwrap();
        while !play_next_in_queue(&ctx.http, &msg.channel_id, &guild.id, &queue, &lava_player).await
        {
        }
    }
    if create_now_playing {
        let handle = create_now_playing_msg(ctx, queue.clone(), msg.channel_id).await?;
        let mut queue_lock = queue.lock().await;
        queue_lock.now_playing_msg = Some(handle);
    }
    handle_autodelete(ctx, msg).await?;

    Ok(())
}
