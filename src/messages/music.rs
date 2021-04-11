use std::sync::Arc;

use serenity::builder::CreateEmbed;
use serenity::http::Http;
use serenity::model::prelude::ChannelId;
use songbird::input::Metadata;

use crate::utils::error::BotResult;
use bot_serenityutils::core::MessageHandle;
use bot_serenityutils::menu::MenuBuilder;
use serenity::builder::CreateMessage;
use serenity::client::Context;
use std::time::Duration;
use tokio::sync::RwLock;

/// Creates a new now playing message and returns the embed for that message
pub async fn create_now_playing_msg(
    ctx: &Context,
    channel_id: ChannelId,
    meta: &Metadata,
) -> BotResult<Arc<RwLock<MessageHandle>>> {
    log::debug!("Creating now playing message");
    let mut page = CreateMessage::default();
    page.embed(|e| create_now_playing_embed(meta, e));

    let handle = MenuBuilder::default()
        .add_page(page)
        .sticky(true)
        .timeout(Duration::from_secs(60 * 60 * 24))
        .build(ctx, channel_id)
        .await?;

    Ok(handle)
}

/// Updates the now playing message with new content
pub async fn update_now_playing_msg(
    http: &Arc<Http>,
    handle: &Arc<RwLock<MessageHandle>>,
    meta: &Metadata,
) -> BotResult<()> {
    log::debug!("Updating now playing message");
    let handle = handle.read().await;
    let mut message = handle.get_message(http).await?;
    message
        .edit(http, |m| m.embed(|e| create_now_playing_embed(meta, e)))
        .await?;

    Ok(())
}

/// Creates the embed of the now playing message
fn create_now_playing_embed<'a>(
    meta: &Metadata,
    mut embed: &'a mut CreateEmbed,
) -> &'a mut CreateEmbed {
    embed = embed.description(format!(
        "Now Playing [{}]({}) by {}",
        meta.title.clone().unwrap(),
        meta.source_url.clone().unwrap(),
        meta.artist.clone().unwrap()
    ));

    if let Some(thumb) = meta.thumbnail.clone() {
        embed = embed.thumbnail(thumb);
    }

    embed
}
