use sea_orm::ActiveValue::Set;
use std::time::SystemTime;

use crate::entity::ephemeral_messages;
use crate::error::DatabaseResult;
use sea_orm::prelude::*;

impl super::BotDatabase {
    /// Adds a command statistic to the database
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn add_ephemeral_message(
        &self,
        channel_id: u64,
        message_id: u64,
        timeout: SystemTime,
    ) -> DatabaseResult<()> {
        let model = ephemeral_messages::ActiveModel {
            channel_id: Set(channel_id as i64),
            message_id: Set(message_id as i64),
            timeout: Set(DateTimeLocal::from(timeout).into()),
            ..Default::default()
        };
        model.insert(&self.db).await?;

        Ok(())
    }

    /// Returns a vec of all ephemeral messages
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn get_ephemeral_messages(&self) -> DatabaseResult<Vec<ephemeral_messages::Model>> {
        let messages = ephemeral_messages::Entity::find().all(&self.db).await?;

        Ok(messages)
    }

    /// Deletes a single ephemeral message
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn delete_ephemeral_message(
        &self,
        channel_id: i64,
        message_id: i64,
    ) -> DatabaseResult<()> {
        ephemeral_messages::Entity::delete_many()
            .filter(ephemeral_messages::Column::ChannelId.eq(channel_id))
            .filter(ephemeral_messages::Column::MessageId.eq(message_id))
            .exec(&self.db)
            .await?;

        Ok(())
    }
}
