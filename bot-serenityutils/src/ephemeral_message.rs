use crate::core::MessageHandle;
use crate::error::SerenityUtilsResult;
use serenity::builder::CreateMessage;
use serenity::http::Http;
use serenity::model::channel::Message;
use serenity::model::id::ChannelId;
use std::sync::Arc;
use std::time::Duration;

pub struct EphemeralMessage;

impl EphemeralMessage {
    /// Ensures that an already existing message is
    /// deleted after a certain amount of time
    pub async fn create_from_message(
        http: &Arc<Http>,
        message: &Message,
        timeout: Duration,
    ) -> SerenityUtilsResult<()> {
        log::debug!("Creating ephemeral message from existing message");
        let handle = MessageHandle::new(message.channel_id, message.id);
        let http = Arc::clone(&http);

        log::debug!("Starting delete task");
        tokio::spawn(async move {
            log::debug!("Waiting for timeout to pass");
            tokio::time::sleep(timeout).await;
            log::debug!("Deleting ephemeral message");
            if let Err(e) = http
                .delete_message(handle.channel_id, handle.message_id)
                .await
            {
                log::error!("Failed to delete ephemeral message {:?}: {}", handle, e);
            }
        });

        Ok(())
    }

    /// Creates a new message that is deleted after a certain amount of time
    pub async fn create<'a, F>(
        http: &Arc<Http>,
        channel_id: ChannelId,
        timeout: Duration,
        f: F,
    ) -> SerenityUtilsResult<Message>
    where
        F: for<'b> FnOnce(&'b mut CreateMessage<'a>) -> &'b mut CreateMessage<'a>,
    {
        log::debug!("Creating new ephemeral message");
        let msg = channel_id.send_message(http, f).await?;
        Self::create_from_message(http, &msg, timeout).await?;

        Ok(msg)
    }
}
