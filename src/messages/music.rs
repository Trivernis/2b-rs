use crate::utils::error::BotResult;
use crate::utils::messages::ShareableMessage;
use serenity::builder::CreateEmbed;
use serenity::http::Http;
use serenity::model::prelude::ChannelId;
use songbird::input::Metadata;
use std::sync::Arc;

#[derive(Clone)]
pub struct NowPlayingMessage {
    inner: ShareableMessage,
}

impl NowPlayingMessage {
    /// Creates a new now playing message
    pub async fn create(
        ctx: Arc<Http>,
        channel_id: &ChannelId,
        meta: &Metadata,
    ) -> BotResult<Self> {
        let inner = ShareableMessage::create(ctx, channel_id, |f| {
            f.embed(|e| Self::create_embed(meta, e))
        })
        .await?;

        Ok(Self { inner })
    }

    /// Returns the inner shareable message
    pub fn inner(&self) -> &ShareableMessage {
        &self.inner
    }

    /// Refreshes the now playing message
    pub async fn refresh(&self, meta: &Metadata) -> BotResult<()> {
        self.inner
            .edit(|m| m.embed(|e| Self::create_embed(meta, e)))
            .await?;

        Ok(())
    }

    /// Creates the embed of the now playing message
    fn create_embed<'a>(meta: &Metadata, mut embed: &'a mut CreateEmbed) -> &'a mut CreateEmbed {
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
}
