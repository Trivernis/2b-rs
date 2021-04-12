use diesel::insert_into;
use diesel::prelude::*;
use tokio_diesel::*;

use crate::error::DatabaseResult;
use crate::models::*;
use crate::schema::*;
use crate::Database;

impl Database {
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
}
