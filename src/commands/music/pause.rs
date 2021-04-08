use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::commands::music::get_queue_for_guild;

#[command]
#[only_in(guilds)]
#[description("Pauses playback")]
#[usage("")]
#[allowed_roles("DJ")]
async fn pause(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    log::debug!("Pausing playback for guild {}", guild.id);
    let queue = get_queue_for_guild(ctx, &guild.id).await?;
    let mut queue_lock = queue.lock().await;

    if let Some(_) = queue_lock.current() {
        queue_lock.pause();
        if queue_lock.paused() {
            log::debug!("Paused");
            msg.channel_id.say(ctx, "Paused playback").await?;
        } else {
            log::debug!("Resumed");
            msg.channel_id.say(ctx, "Resumed playback").await?;
        }
    } else {
        msg.channel_id.say(ctx, "Nothing to pause").await?;
    }

    Ok(())
}
