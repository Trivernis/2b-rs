use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

use crate::commands::common::handle_autodelete;
use crate::commands::music::{get_queue_for_guild, DJ_CHECK};
use bot_serenityutils::core::SHORT_TIMEOUT;
use bot_serenityutils::ephemeral_message::EphemeralMessage;

#[command]
#[only_in(guilds)]
#[description("Shuffles the queue")]
#[usage("")]
#[aliases("sh")]
#[bucket("general")]
#[checks(DJ)]
async fn shuffle(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();

    log::debug!("Shuffling queue for guild {}", guild.id);
    let queue = forward_error!(
        ctx,
        msg.channel_id,
        get_queue_for_guild(ctx, &guild.id).await
    );
    {
        let mut queue_lock = queue.lock().await;
        queue_lock.shuffle();
    }

    EphemeralMessage::create(&ctx.http, msg.channel_id, SHORT_TIMEOUT, |m| {
        m.content("🔀 The queue has been shuffled")
    })
    .await?;
    handle_autodelete(ctx, msg).await?;

    Ok(())
}
