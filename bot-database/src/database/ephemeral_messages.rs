use std::time::SystemTime;

use diesel::prelude::*;
use diesel::{delete, insert_into};
use tokio_diesel::*;

use crate::error::DatabaseResult;
use crate::models::*;
use crate::schema::*;
use crate::Database;

impl Database {
    /// Adds a command statistic to the database
    pub async fn add_ephemeral_message(
        &self,
        channel_id: u64,
        message_id: u64,
        timeout: SystemTime,
    ) -> DatabaseResult<()> {
        use ephemeral_messages::dsl;
        insert_into(dsl::ephemeral_messages)
            .values(EphemeralMessageInsert {
                channel_id: channel_id as i64,
                message_id: message_id as i64,
                timeout,
            })
            .execute_async(&self.pool)
            .await?;

        Ok(())
    }

    /// Returns a vec of all ephemeral messages
    pub async fn get_ephemeral_messages(&self) -> DatabaseResult<Vec<EphemeralMessage>> {
        use ephemeral_messages::dsl;
        let messages: Vec<EphemeralMessage> = dsl::ephemeral_messages
            .load_async::<EphemeralMessage>(&self.pool)
            .await?;

        Ok(messages)
    }

    /// Deletes a single ephemeral message
    pub async fn delete_ephemeral_message(
        &self,
        channel_id: i64,
        message_id: i64,
    ) -> DatabaseResult<()> {
        use ephemeral_messages::dsl;
        delete(dsl::ephemeral_messages)
            .filter(dsl::channel_id.eq(channel_id))
            .filter(dsl::message_id.eq(message_id))
            .execute_async(&self.pool)
            .await?;

        Ok(())
    }
}
