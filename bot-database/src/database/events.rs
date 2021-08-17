use std::time::SystemTime;

use diesel::insert_into;
use diesel::prelude::*;
use tokio_diesel::*;

use crate::error::DatabaseResult;
use crate::models::*;
use crate::schema::*;
use crate::Database;
use diesel::dsl::now;

impl Database {
    /// Adds a command statistic to the database
    pub async fn add_event(
        &self,
        guild_id: u64,
        channel_id: u64,
        name: String,
        description: String,
        event_start: SystemTime,
        event_end: Option<SystemTime>,
    ) -> DatabaseResult<()> {
        use events::dsl;
        log::trace!("Adding event to database");
        insert_into(dsl::events)
            .values(EventInsert {
                guild_id: guild_id as i64,
                channel_id: channel_id as i64,
                name,
                description,
                event_start,
                event_end,
            })
            .execute_async(&self.pool)
            .await?;

        Ok(())
    }

    /// Returns events for a given guild started before a given timestamp
    pub async fn get_events_started_before(
        &self,
        time: SystemTime,
        guild_id: u64,
    ) -> DatabaseResult<Vec<Event>> {
        use events::dsl;
        log::trace!("Querying events before {:?}", time);
        let events: Vec<Event> = dsl::events
            .filter(dsl::event_start.lt(time))
            .filter(dsl::guild_id.eq(guild_id as i64))
            .load_async::<Event>(&self.pool)
            .await?;
        Ok(events)
    }

    /// Returns events for a given guild started after a given timestamp
    pub async fn get_events_starting_after(
        &self,
        time: SystemTime,
        guild_id: u64,
    ) -> DatabaseResult<Vec<Event>> {
        use events::dsl;
        log::trace!("Querying events after {:?}", time);
        let events: Vec<Event> = dsl::events
            .filter(dsl::event_start.gt(time))
            .filter(dsl::guild_id.eq(guild_id as i64))
            .load_async::<Event>(&self.pool)
            .await?;

        Ok(events)
    }

    /// Deletes events that completed in the past
    pub async fn delete_completed_events(&self) -> DatabaseResult<()> {
        use events::dsl;
        log::trace!("Deleting completed events");
        diesel::delete(dsl::events.filter(dsl::event_end.lt(now)))
            .execute_async(&self.pool)
            .await?;

        Ok(())
    }
}
