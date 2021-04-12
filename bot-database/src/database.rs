use crate::error::DatabaseResult;
use crate::models::*;
use crate::schema::*;
use crate::PoolConnection;
use diesel::prelude::*;
use diesel::{delete, insert_into};
use std::any;
use std::fmt::Debug;
use std::str::FromStr;
use std::time::SystemTime;
use tokio_diesel::*;

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

    /// Returns a list of all guild playlists
    pub async fn get_guild_playlists(&self, guild_id: u64) -> DatabaseResult<Vec<GuildPlaylist>> {
        use guild_playlists::dsl;
        log::debug!("Retrieving guild playlists for guild {}", guild_id);

        let playlists: Vec<GuildPlaylist> = dsl::guild_playlists
            .filter(dsl::guild_id.eq(guild_id as i64))
            .load_async::<GuildPlaylist>(&self.pool)
            .await?;

        Ok(playlists)
    }

    /// Returns a guild playlist by name
    pub async fn get_guild_playlist(
        &self,
        guild_id: u64,
        name: String,
    ) -> DatabaseResult<Option<GuildPlaylist>> {
        use guild_playlists::dsl;
        log::debug!("Retriving guild playlist '{}' for guild {}", name, guild_id);

        let playlists: Vec<GuildPlaylist> = dsl::guild_playlists
            .filter(dsl::guild_id.eq(guild_id as i64))
            .filter(dsl::name.eq(name))
            .load_async::<GuildPlaylist>(&self.pool)
            .await?;

        Ok(playlists.into_iter().next())
    }

    /// Adds a new playlist to the database overwriting the old one
    pub async fn add_guild_playlist(
        &self,
        guild_id: u64,
        name: String,
        url: String,
    ) -> DatabaseResult<()> {
        use guild_playlists::dsl;
        log::debug!("Inserting guild playlist '{}' for guild {}", name, guild_id);

        insert_into(dsl::guild_playlists)
            .values(GuildPlaylistInsert {
                guild_id: guild_id as i64,
                name: name.clone(),
                url: url.clone(),
            })
            .on_conflict((dsl::guild_id, dsl::name))
            .do_update()
            .set(dsl::url.eq(url))
            .execute_async(&self.pool)
            .await?;

        Ok(())
    }

    /// Returns a list of all gifs in the database
    pub async fn get_all_gifs(&self) -> DatabaseResult<Vec<Gif>> {
        use gifs::dsl;
        log::debug!("Loading all gifs from the database");

        let gifs: Vec<Gif> = dsl::gifs.load_async::<Gif>(&self.pool).await?;
        Ok(gifs)
    }

    /// Returns a list of gifs by assigned category
    pub async fn get_gifs_by_category(&self, category: &str) -> DatabaseResult<Vec<Gif>> {
        use gifs::dsl;
        log::debug!("Searching for gifs in category '{}'", category);

        let gifs: Vec<Gif> = dsl::gifs
            .filter(dsl::category.eq(category))
            .load_async::<Gif>(&self.pool)
            .await?;
        Ok(gifs)
    }

    /// Adds a gif to the database
    pub async fn add_gif(
        &self,
        url: &str,
        category: Option<String>,
        name: Option<String>,
    ) -> DatabaseResult<()> {
        use gifs::dsl;
        log::debug!(
            "Inserting gif with url '{}' and name {:?} and category {:?}",
            url,
            name,
            category
        );
        insert_into(dsl::gifs)
            .values(GifInsert {
                url: url.to_string(),
                name,
                category,
            })
            .execute_async(&self.pool)
            .await?;

        Ok(())
    }

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
            .values(StatisticsInsert {
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
}
