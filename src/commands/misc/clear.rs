use futures::future::BoxFuture;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;
use serenity::Result as SerenityResult;
use serenity_rich_interaction::core::SHORT_TIMEOUT;
use serenity_rich_interaction::ephemeral_message::EphemeralMessage;

#[command]
#[description("Clears the chat (maximum 100 messages)")]
#[usage("[<number>]")]
#[example("20")]
#[min_args(0)]
#[max_args(1)]
#[bucket("general")]
#[required_permissions("MANAGE_MESSAGES")]
async fn clear(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let limit = args.single::<u64>().unwrap_or(20);
    tracing::debug!("Deleting messages for channel {}", msg.channel_id);
    let messages = msg.channel_id.messages(ctx, |b| b.limit(limit)).await?;
    tracing::debug!("Deleting {} messages", messages.len());
    let futures: Vec<BoxFuture<SerenityResult<()>>> = messages
        .into_iter()
        .map(|m| async move { ctx.http.delete_message(m.channel_id.0, m.id.0).await }.boxed())
        .collect();
    tracing::debug!("Waiting for all messages to be deleted");
    let deleted = futures::future::join_all(futures).await;
    let deleted_count = deleted.into_iter().filter(|d| d.is_ok()).count();
    tracing::debug!("{} Messages deleted", deleted_count);

    EphemeralMessage::create(&ctx.http, msg.channel_id, SHORT_TIMEOUT, |f| {
        f.content(format!("Deleted {} messages", deleted_count))
    })
    .await?;

    Ok(())
}
