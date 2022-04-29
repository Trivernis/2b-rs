use crate::entity::youtube_songs;
use crate::error::DatabaseResult;
use sea_orm::prelude::*;
use sea_orm::ActiveValue::Set;

impl super::BotDatabase {
    /// Adds a song to the database or increments the score when it
    /// already exists
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn add_song(
        &self,
        spotify_id: String,
        artist: String,
        title: String,
        album: String,
        url: String,
    ) -> DatabaseResult<()> {
        if let Some(model) = self.get_song(&spotify_id).await? {
            let mut active_model: youtube_songs::ActiveModel = model.into();
            active_model.score = Set(active_model.score.unwrap() + 1);
            active_model.update(&self.db).await?;
        } else {
            let model = youtube_songs::ActiveModel {
                spotify_id: Set(spotify_id),
                artist: Set(artist),
                title: Set(title),
                album: Set(album),
                url: Set(url),
                ..Default::default()
            };
            model.insert(&self.db).await?;
        }

        Ok(())
    }

    /// Returns the song with the best score for the given query
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn get_song(&self, spotify_id: &str) -> DatabaseResult<Option<youtube_songs::Model>> {
        let song = youtube_songs::Entity::find()
            .filter(youtube_songs::Column::SpotifyId.eq(spotify_id))
            .one(&self.db)
            .await?;

        Ok(song)
    }

    /// Deletes a song from the database
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn delete_song(&self, id: i64) -> DatabaseResult<()> {
        youtube_songs::Entity::delete_many()
            .filter(youtube_songs::Column::Id.eq(id))
            .exec(&self.db)
            .await?;

        Ok(())
    }
}
