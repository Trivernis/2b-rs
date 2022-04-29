use crate::entity::guild_playlists;
use crate::error::DatabaseResult;
use sea_orm::prelude::*;
use sea_orm::ActiveValue::Set;

impl super::BotDatabase {
    /// Returns a list of all guild playlists
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn get_guild_playlists(
        &self,
        guild_id: u64,
    ) -> DatabaseResult<Vec<guild_playlists::Model>> {
        let playlists = guild_playlists::Entity::find()
            .filter(guild_playlists::Column::GuildId.eq(guild_id))
            .all(&self.db)
            .await?;

        Ok(playlists)
    }

    /// Returns a guild playlist by name
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn get_guild_playlist(
        &self,
        guild_id: u64,
        name: String,
    ) -> DatabaseResult<Option<guild_playlists::Model>> {
        let playlist = guild_playlists::Entity::find()
            .filter(guild_playlists::Column::GuildId.eq(guild_id))
            .filter(guild_playlists::Column::Name.eq(name))
            .one(&self.db)
            .await?;

        Ok(playlist)
    }

    /// Adds a new playlist to the database overwriting the old one
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn add_guild_playlist(
        &self,
        guild_id: u64,
        name: String,
        url: String,
    ) -> DatabaseResult<()> {
        let model = guild_playlists::ActiveModel {
            guild_id: Set(guild_id as i64),
            name: Set(name),
            url: Set(url),
            ..Default::default()
        };
        model.insert(&self.db).await?;

        Ok(())
    }
}
