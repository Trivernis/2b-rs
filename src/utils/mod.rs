use crate::utils::error::BotResult;
use serenity::client::Context;
use serenity::model::channel::Message;

pub(crate) mod context_data;
pub(crate) mod error;
pub(crate) mod logging;
pub(crate) mod messages;

/// Returns the message the given message is a reply to or the message sent before that
pub async fn get_previous_message_or_reply(
    ctx: &Context,
    msg: &Message,
) -> BotResult<Option<Message>> {
    let referenced = if let Some(reference) = &msg.referenced_message {
        Some(*reference.clone())
    } else {
        let messages = msg
            .channel_id
            .messages(ctx, |ret| ret.before(&msg.id).limit(1))
            .await?;
        messages.first().cloned()
    };

    Ok(referenced)
}
