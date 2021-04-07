use crate::utils::error::{BotError, BotResult};
use serenity::client::Context;
use serenity::model::guild::Guild;
use serenity::model::id::{ChannelId, GuildId, UserId};
use songbird::Call;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Joins a voice channel
pub(crate) async fn join_channel(
    ctx: &Context,
    channel_id: ChannelId,
    guild_id: GuildId,
) -> Arc<Mutex<Call>> {
    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let (handler, _) = manager.join(guild_id, channel_id).await;
    handler
}

/// Returns the voice channel the author is in
pub(crate) fn get_channel_for_author(author_id: &UserId, guild: &Guild) -> BotResult<ChannelId> {
    guild
        .voice_states
        .get(author_id)
        .and_then(|voice_state| voice_state.channel_id)
        .ok_or(BotError::from("Not in a voice channel."))
}
