use crate::entity::statistics;
use crate::error::DatabaseResult;
use sea_orm::prelude::*;
use sea_orm::ActiveValue::Set;
use sea_orm::{FromQueryResult, QuerySelect};
use std::time::SystemTime;

#[derive(FromQueryResult)]
struct CommandCount {
    count: i64,
}

impl super::BotDatabase {
    /// Adds a command statistic to the database
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn add_statistic(
        &self,
        version: String,
        command: String,
        executed_at: SystemTime,
        success: bool,
        error_msg: Option<String>,
    ) -> DatabaseResult<()> {
        let model = statistics::ActiveModel {
            version: Set(version),
            command: Set(command),
            executed_at: Set(DateTimeLocal::from(executed_at).into()),
            success: Set(success),
            error_msg: Set(error_msg),
            ..Default::default()
        };
        model.insert(&self.db).await?;

        Ok(())
    }

    /// Returns the total number of commands executed
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn get_total_commands_statistic(&self) -> DatabaseResult<u64> {
        let total_count: Option<CommandCount> = statistics::Entity::find()
            .select_only()
            .column_as(statistics::Column::Id.count(), "count")
            .into_model::<CommandCount>()
            .one(&self.db)
            .await?;

        Ok(total_count.unwrap().count as u64)
    }
}
