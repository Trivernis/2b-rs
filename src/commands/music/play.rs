use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandError, CommandResult};
use serenity::model::channel::Message;

use crate::commands::music::{
    get_channel_for_author, get_queue_for_guild, get_songs_for_query, get_voice_manager,
    join_channel, play_next_in_queue,
};
use crate::database::get_database_from_context;
use crate::database::guild::SETTING_AUTOSHUFFLE;

#[command]
#[only_in(guilds)]
#[description("Plays a song in a voice channel")]
#[usage("play <url>")]
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
        let database = get_database_from_context(ctx).await;
        let database_lock = database.lock().await;
        let autoshuffle = database_lock
            .get_guild_setting(&guild.id, SETTING_AUTOSHUFFLE)
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

    Ok(())
}
