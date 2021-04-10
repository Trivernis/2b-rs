use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandError, CommandResult};
use serenity::model::channel::Message;

use crate::commands::common::handle_autodelete;
use crate::commands::music::{
    get_channel_for_author, get_queue_for_guild, get_songs_for_query, get_voice_manager,
    join_channel, play_next_in_queue,
};
use crate::providers::settings::{get_setting, Setting};

#[command]
#[only_in(guilds)]
#[description("Plays a song in a voice channel")]
#[usage("(<spotify_url,youtube_url,query>)")]
#[min_args(1)]
#[aliases("p")]
async fn play(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let query = args.message();

    let guild = msg.guild(&ctx.cache).await.unwrap();
    log::debug!("Play request received for guild {}", guild.id);

    let manager = get_voice_manager(ctx).await;
    let mut handler = manager.get(guild.id);

    if handler.is_none() {
        log::debug!("Not in a channel. Joining authors channel...");
        msg.guild(&ctx.cache).await.unwrap();
        let channel_id = get_channel_for_author(&msg.author.id, &guild)?;
        handler = Some(join_channel(ctx, channel_id, guild.id).await);
    }

    let handler_lock = handler.ok_or(CommandError::from("Not in a voice channel"))?;

    let songs = get_songs_for_query(&ctx, msg, query).await?;

    let queue = get_queue_for_guild(ctx, &guild.id).await?;

    let play_first = {
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
        queue_lock.current().is_none()
    };

    if play_first {
        log::debug!("Playing first song in queue");
        while !play_next_in_queue(&ctx.http, &msg.channel_id, &queue, &handler_lock).await {}
    }
    handle_autodelete(ctx, msg).await?;

    Ok(())
}
