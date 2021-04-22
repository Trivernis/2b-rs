use crate::utils::error::BotResult;
use serenity::http::Http;
use serenity::model::prelude::ChannelId;
use serenity_rich_interaction::core::SHORT_TIMEOUT;
use serenity_rich_interaction::ephemeral_message::EphemeralMessage;
use std::sync::Arc;

/// Creates a not in a voicechannel message
pub async fn create_no_voicechannel_message(
    http: &Arc<Http>,
    channel_id: ChannelId,
) -> BotResult<()> {
    EphemeralMessage::create(http, channel_id, SHORT_TIMEOUT, |m| {
        m.content("‼️ I'm not in a Voice Channel")
    })
    .await?;

    Ok(())
}
