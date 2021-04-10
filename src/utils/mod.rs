use crate::utils::error::BotResult;
use rand::Rng;
use serenity::client::Context;
use serenity::model::channel::Message;
use std::collections::VecDeque;

pub(crate) mod context_data;
pub(crate) mod error;
pub(crate) mod logging;
pub(crate) mod messages;

/// Fisher-Yates shuffle for VecDeque
pub fn shuffle_vec_deque<T>(deque: &mut VecDeque<T>) {
    let mut rng = rand::thread_rng();
    let mut i = deque.len();
    while i >= 2 {
        i -= 1;
        deque.swap(i, rng.gen_range(0..i + 1))
    }
}

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
