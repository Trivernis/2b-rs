use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

use crate::commands::music::get_queue_for_guild;

#[command]
#[only_in(guilds)]
#[description("Shuffles the queue")]
#[usage("shuffle")]
#[aliases("sh")]
#[allowed_roles("DJ")]
async fn shuffle(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();

    log::debug!("Shuffling queue for guild {}", guild.id);
    let queue = get_queue_for_guild(ctx, &guild.id).await?;
    {
        let mut queue_lock = queue.lock().await;
        queue_lock.shuffle();
    }

    msg.channel_id
        .say(ctx, "The queue has been shuffled")
        .await?;

    Ok(())
}
