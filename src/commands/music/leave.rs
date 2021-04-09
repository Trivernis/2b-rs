use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

use crate::commands::common::handle_autodelete;
use crate::commands::music::{get_queue_for_guild, get_voice_manager};

#[command]
#[only_in(guilds)]
#[description("Leaves a voice channel")]
#[usage("")]
#[aliases("stop")]
#[allowed_roles("DJ")]
async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    log::debug!("Leave request received for guild {}", guild.id);
    let manager = get_voice_manager(ctx).await;
    let queue = get_queue_for_guild(ctx, &guild.id).await?;
    let queue_lock = queue.lock().await;
    let handler = manager.get(guild.id);

    if let Some(handler) = handler {
        let mut handler_lock = handler.lock().await;
        handler_lock.remove_all_global_events();
    }
    if let Some(current) = queue_lock.current() {
        current.stop()?;
    }

    if manager.get(guild.id).is_some() {
        manager.remove(guild.id).await?;
        msg.channel_id.say(ctx, "Left the voice channel").await?;
        log::debug!("Left the voice channel");
    } else {
        msg.channel_id.say(ctx, "Not in a voice channel").await?;
        log::debug!("Not in a voice channel");
    }
    handle_autodelete(ctx, msg).await?;

    Ok(())
}
