use crate::error::DatabaseResult;
use crate::models::*;
use crate::schema::*;
use crate::PoolConnection;
use diesel::insert_into;
use diesel::prelude::*;
use std::any;
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Clone)]
pub struct Database {
    pool: PoolConnection,
}

unsafe impl Send for Database {}
unsafe impl Sync for Database {}

impl Database {
    pub fn new(pool: PoolConnection) -> Self {
        Self { pool }
    }

    /// Returns a guild setting from the database
    pub fn get_guild_setting<T: 'static>(
        &self,
        guild_id: u64,
        key: &str,
    ) -> DatabaseResult<Option<T>>
    where
        T: FromStr,
    {
        use guild_settings::dsl;
        log::debug!("Retrieving setting '{}' for guild {}", key, guild_id);
        let connection = self.pool.get()?;

        let entries: Vec<GuildSetting> = dsl::guild_settings
            .filter(dsl::guild_id.eq(guild_id as i64))
            .filter(dsl::key.eq(key))
            .load::<GuildSetting>(&connection)?;
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
    pub fn set_guild_setting<T>(&self, guild_id: u64, key: &str, value: T) -> DatabaseResult<()>
    where
        T: ToString + Debug,
    {
        use guild_settings::dsl;
        log::debug!("Setting '{}' to '{:?}' for guild {}", key, value, guild_id);
        let connection = self.pool.get()?;

        insert_into(dsl::guild_settings)
            .values(&GuildSettingInsert {
                guild_id: guild_id as i64,
                key: key.to_string(),
                value: value.to_string(),
            })
            .on_conflict((dsl::guild_id, dsl::key))
            .do_update()
            .set(dsl::value.eq(value.to_string()))
            .execute(&connection)?;

        Ok(())
    }
}
