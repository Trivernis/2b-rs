use std::sync::Arc;

use serenity::builder::CreateEmbed;
use serenity::http::Http;
use serenity::model::prelude::ChannelId;
use songbird::input::Metadata;

use crate::utils::error::BotResult;
use crate::utils::messages::ShareableMessage;

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
    pub async fn refresh(&mut self, meta: &Metadata) -> BotResult<()> {
        let channel_id = self.inner.channel_id();
        let messages = channel_id
            .messages(self.inner.http(), |p| p.limit(1))
            .await?;

        let needs_recreate = messages
            .first()
            .map(|m| m.id != self.inner.message_id())
            .unwrap_or(true);

        // recreates the message if needed
        if needs_recreate {
            log::debug!("Song info message will be recreated");
            let http = self.inner.http().clone();
            let _ = self.inner.delete().await;

            self.inner = ShareableMessage::create(http, &channel_id, |f| {
                f.embed(|e| Self::create_embed(meta, e))
            })
            .await?;
        } else {
            log::debug!("Reusing old song info");
            self.inner
                .edit(|m| m.embed(|e| Self::create_embed(meta, e)))
                .await?;
        }

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
