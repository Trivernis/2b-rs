use std::any;
use std::fmt::Debug;
use std::str::FromStr;

use diesel::prelude::*;
use diesel::{delete, insert_into};
use tokio_diesel::*;

use crate::error::DatabaseResult;
use crate::models::*;
use crate::schema::*;
use crate::Database;

impl Database {
    /// Returns a guild setting from the database
    pub async fn get_guild_setting<T: 'static>(
        &self,
        guild_id: u64,
        key: String,
    ) -> DatabaseResult<Option<T>>
    where
        T: FromStr,
    {
        use guild_settings::dsl;
        log::debug!("Retrieving setting '{}' for guild {}", key, guild_id);

        let entries: Vec<GuildSetting> = dsl::guild_settings
            .filter(dsl::guild_id.eq(guild_id as i64))
            .filter(dsl::key.eq(key))
            .load_async::<GuildSetting>(&self.pool)
            .await?;
        log::trace!("Result is {:?}", entries);

        if let Some(first) = entries.first() {
            if any::TypeId::of::<T>() == any::TypeId::of::<bool>() {
                Ok(first
                    .value
                    .clone()
                    .unwrap_or("false".to_string())
                    .parse::<T>()
                    .ok())
            } else {
                Ok(first.value.clone().and_then(|v| v.parse::<T>().ok()))
            }
        } else {
            return Ok(None);
        }
    }

    /// Upserting a guild setting
    pub async fn set_guild_setting<T>(
        &self,
        guild_id: u64,
        key: String,
        value: T,
    ) -> DatabaseResult<()>
    where
        T: ToString + Debug,
    {
        use guild_settings::dsl;
        log::debug!("Setting '{}' to '{:?}' for guild {}", key, value, guild_id);

        insert_into(dsl::guild_settings)
            .values(GuildSettingInsert {
                guild_id: guild_id as i64,
                key: key.to_string(),
                value: value.to_string(),
            })
            .on_conflict((dsl::guild_id, dsl::key))
            .do_update()
            .set(dsl::value.eq(value.to_string()))
            .execute_async(&self.pool)
            .await?;

        Ok(())
    }

    /// Deletes a guild setting
    pub async fn delete_guild_setting(&self, guild_id: u64, key: String) -> DatabaseResult<()> {
        use guild_settings::dsl;
        delete(dsl::guild_settings)
            .filter(dsl::guild_id.eq(guild_id as i64))
            .filter(dsl::key.eq(key))
            .execute_async(&self.pool)
            .await?;

        Ok(())
    }
}
