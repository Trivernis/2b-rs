use crate::utils::context_data::get_database_from_context;
use crate::utils::error::BotResult;
use bot_serenityutils::core::MessageHandle;
use serenity::client::Context;
use std::time::{Duration, SystemTime};

pub mod gifs;
pub mod music;
pub mod sauce;

/// Adds an ephemeral message to the database
pub async fn add_ephemeral_handle_to_database(
    ctx: &Context,
    handle: MessageHandle,
    timeout: Duration,
) -> BotResult<()> {
    let timeout = SystemTime::now() + timeout;
    let database = get_database_from_context(ctx).await;
    database
        .add_ephemeral_message(handle.channel_id, handle.message_id, timeout)
        .await?;

    Ok(())
}
