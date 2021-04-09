use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

use crate::commands::common::handle_autodelete;
use crate::commands::music::{get_queue_for_guild, is_dj};

#[command]
#[only_in(guilds)]
#[description("Skips to the next song")]
#[usage("")]
#[aliases("next")]
async fn skip(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    if !is_dj(ctx, guild.id, &msg.author).await? {
        msg.channel_id.say(ctx, "Requires DJ permissions").await?;
        return Ok(());
    }
    log::debug!("Skipping song for guild {}", guild.id);
    let queue = get_queue_for_guild(ctx, &guild.id).await?;
    let queue_lock = queue.lock().await;

    if let Some(current) = queue_lock.current() {
        current.stop()?;
    }

    msg.channel_id.say(ctx, "Skipped to the next song").await?;
    handle_autodelete(ctx, msg).await?;

    Ok(())
}
