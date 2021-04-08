use serenity::client::Context;
use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::command;
use serenity::model::channel::Message;

use crate::commands::music::get_queue_for_guild;

#[command]
#[only_in(guilds)]
#[description("Clears the queue")]
#[usage("clear")]
#[aliases("cl")]
#[allowed_roles("DJ")]
async fn clear(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();

    let queue = get_queue_for_guild(ctx, &guild.id).await?;
    {
        let mut queue_lock = queue.lock().await;
        queue_lock.clear();
    }

    msg.channel_id
        .say(ctx, "The queue has been cleared")
        .await?;

    Ok(())
}
