use crate::utils::error::BotResult;
use serenity::async_trait;
use serenity::builder::{CreateMessage, EditMessage};
use serenity::http::{CacheHttp, Http};
use serenity::model::channel::{Message, Reaction};
use serenity::model::id::{ChannelId, MessageId};
use std::sync::Arc;

#[async_trait]
pub trait EventDrivenMessage: Send + Sync {
    async fn on_deleted(&self) -> BotResult<()>;
    async fn on_reaction_add(&self, reaction: Reaction) -> BotResult<()>;
    async fn on_reaction_remove(&self, reaction: Reaction) -> BotResult<()>;
}

#[derive(Clone)]
pub struct ShareableMessage {
    http: Arc<Http>,
    channel_id: u64,
    message_id: u64,
}

impl ShareableMessage {
    /// Creates a new active message
    pub async fn create<'a, F>(http: Arc<Http>, channel_id: &ChannelId, f: F) -> BotResult<Self>
    where
        for<'b> F: FnOnce(&'b mut CreateMessage<'a>) -> &'b mut CreateMessage<'a>,
    {
        let msg = channel_id.send_message(http.http(), f).await?;

        Ok(Self::new(http, &msg.channel_id, &msg.id))
    }

    /// Creates a new active message
    pub fn new(http: Arc<Http>, channel_id: &ChannelId, message_id: &MessageId) -> Self {
        Self {
            http,
            channel_id: channel_id.0,
            message_id: message_id.0,
        }
    }

    /// Deletes the underlying message
    pub async fn delete(&self) -> BotResult<()> {
        let msg = self.get_message().await?;
        msg.delete(&self.http).await?;

        Ok(())
    }

    /// Edits the active message
    pub async fn edit<F>(&self, f: F) -> BotResult<()>
    where
        F: FnOnce(&mut EditMessage) -> &mut EditMessage,
    {
        let mut message = self.get_message().await?;
        message.edit(&self.http, f).await?;

        Ok(())
    }

    /// Returns the underlying message
    async fn get_message(&self) -> BotResult<Message> {
        let message = self
            .http
            .http()
            .get_message(self.channel_id, self.message_id)
            .await?;

        Ok(message)
    }
}
