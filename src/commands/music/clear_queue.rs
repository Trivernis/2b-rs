use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

use crate::commands::common::handle_autodelete;
use crate::commands::music::{get_queue_for_guild, is_dj};

#[command]
#[only_in(guilds)]
#[description("Clears the queue")]
#[usage("")]
#[aliases("cq", "clear-queue", "clearqueue")]
#[bucket("general")]
async fn clear_queue(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();

    if !is_dj(ctx, guild.id, &msg.author).await? {
        msg.channel_id.say(ctx, "Requires DJ permissions").await?;
        return Ok(());
    }
    log::debug!("Clearing queue for guild {}", guild.id);

    let queue = get_queue_for_guild(ctx, &guild.id).await?;
    {
        let mut queue_lock = queue.lock().await;
        queue_lock.clear();
    }

    msg.channel_id
        .say(ctx, "The queue has been cleared")
        .await?;
    handle_autodelete(ctx, msg).await?;

    Ok(())
}
