use std::sync::Arc;

use serenity::builder::{CreateMessage, EditMessage};
use serenity::http::{CacheHttp, Http};
use serenity::model::channel::Message;
use serenity::model::id::{ChannelId, MessageId};

use crate::utils::error::BotResult;

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
        let msg = self.get_discord_message().await?;
        msg.delete(&self.http).await?;

        Ok(())
    }

    /// Edits the active message
    pub async fn edit<F>(&self, f: F) -> BotResult<()>
    where
        F: FnOnce(&mut EditMessage) -> &mut EditMessage,
    {
        let mut message = self.get_discord_message().await?;
        message.edit(&self.http, f).await?;

        Ok(())
    }

    /// Returns the underlying message
    pub async fn get_discord_message(&self) -> BotResult<Message> {
        let message = self
            .http
            .get_message(self.channel_id, self.message_id)
            .await?;

        Ok(message)
    }

    /// Returns the channel of the message
    pub fn channel_id(&self) -> ChannelId {
        ChannelId(self.channel_id)
    }

    /// Returns the message id of the message
    pub fn message_id(&self) -> MessageId {
        MessageId(self.message_id)
    }

    /// Returns the reference to the http object
    pub fn http(&self) -> &Arc<Http> {
        &self.http
    }
}
