use std::time::SystemTime;

use diesel::dsl::count;
use diesel::insert_into;
use diesel::prelude::*;
use tokio_diesel::*;

use crate::error::DatabaseResult;
use crate::models::*;
use crate::schema::*;
use crate::Database;

impl Database {
    /// Adds a command statistic to the database
    pub async fn add_statistic(
        &self,
        version: &str,
        command: &str,
        executed_at: SystemTime,
        success: bool,
        error_msg: Option<String>,
    ) -> DatabaseResult<()> {
        use statistics::dsl;
        log::trace!("Adding statistic to database");
        insert_into(dsl::statistics)
            .values(StatisticInsert {
                version: version.to_string(),
                command: command.to_string(),
                executed_at,
                success,
                error_msg,
            })
            .execute_async(&self.pool)
            .await?;

        Ok(())
    }

    /// Returns the total number of commands executed
    pub async fn get_total_commands_statistic(&self) -> DatabaseResult<u64> {
        use statistics::dsl;
        log::trace!("Querying total number of commands");
        let total_count: i64 = dsl::statistics
            .select(count(dsl::id))
            .first_async::<i64>(&self.pool)
            .await?;

        Ok(total_count as u64)
    }
}
