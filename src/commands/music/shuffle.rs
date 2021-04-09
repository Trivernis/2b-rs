use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

use crate::commands::common::handle_autodelete;
use crate::commands::music::{get_queue_for_guild, is_dj};

#[command]
#[only_in(guilds)]
#[description("Shuffles the queue")]
#[usage("")]
#[aliases("sh")]
async fn shuffle(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();

    log::debug!("Shuffling queue for guild {}", guild.id);
    if !is_dj(ctx, guild.id, &msg.author).await? {
        msg.channel_id.say(ctx, "Requires DJ permissions").await?;
        return Ok(());
    }
    let queue = get_queue_for_guild(ctx, &guild.id).await?;
    {
        let mut queue_lock = queue.lock().await;
        queue_lock.shuffle();
    }

    msg.channel_id
        .say(ctx, "The queue has been shuffled")
        .await?;
    handle_autodelete(ctx, msg).await?;

    Ok(())
}
