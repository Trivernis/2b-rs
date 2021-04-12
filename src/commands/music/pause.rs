use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::commands::common::handle_autodelete;
use crate::commands::music::{get_queue_for_guild, is_dj};
use crate::messages::music::now_playing::update_now_playing_msg;

#[command]
#[only_in(guilds)]
#[description("Pauses playback")]
#[usage("")]
#[bucket("general")]
async fn pause(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    log::debug!("Pausing playback for guild {}", guild.id);
    if !is_dj(ctx, guild.id, &msg.author).await? {
        msg.channel_id.say(ctx, "Requires DJ permissions").await?;
        return Ok(());
    }

    let queue = get_queue_for_guild(ctx, &guild.id).await?;
    let mut queue_lock = queue.lock().await;

    if let Some(_) = queue_lock.current() {
        queue_lock.pause();
        if queue_lock.paused() {
            log::debug!("Paused");
            msg.channel_id.say(ctx, "Paused playback").await?;
            if let (Some(menu), Some(current)) = (&queue_lock.now_playing_msg, queue_lock.current())
            {
                update_now_playing_msg(&ctx.http, menu, current.metadata(), true).await?;
            }
        } else {
            log::debug!("Resumed");
            msg.channel_id.say(ctx, "Resumed playback").await?;
            if let (Some(menu), Some(current)) = (&queue_lock.now_playing_msg, queue_lock.current())
            {
                update_now_playing_msg(&ctx.http, menu, current.metadata(), true).await?;
            }
        }
    } else {
        msg.channel_id.say(ctx, "Nothing to pause").await?;
    }
    handle_autodelete(ctx, msg).await?;

    Ok(())
}
