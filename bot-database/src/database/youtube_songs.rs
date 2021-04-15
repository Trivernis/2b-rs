use diesel::prelude::*;
use diesel::{delete, insert_into};
use tokio_diesel::*;

use crate::error::DatabaseResult;
use crate::models::*;
use crate::schema::*;
use crate::Database;

impl Database {
    /// Adds a song to the database or increments the score when it
    /// already exists
    pub async fn add_song(
        &self,
        spotify_id: &str,
        artist: &str,
        title: &str,
        album: &str,
        url: &str,
    ) -> DatabaseResult<()> {
        use youtube_songs::dsl;
        log::debug!(
            "Inserting/Updating song in database spotify_id: '{}' artist: '{}', title: '{}', album: '{}', url: '{}'",
            spotify_id,
            artist,
            title,
            album,
            url,
        );

        insert_into(dsl::youtube_songs)
            .values(YoutubeSongInsert {
                spotify_id: spotify_id.to_string(),
                artist: artist.to_string(),
                title: title.to_string(),
                album: album.to_string(),
                url: url.to_string(),
            })
            .on_conflict((dsl::spotify_id, dsl::url))
            .do_update()
            .set(dsl::score.eq(dsl::score + 1))
            .execute_async(&self.pool)
            .await?;

        Ok(())
    }

    /// Returns the song with the best score for the given query
    pub async fn get_song(&self, spotify_id: &str) -> DatabaseResult<Option<YoutubeSong>> {
        use youtube_songs::dsl;
        let songs: Vec<YoutubeSong> = dsl::youtube_songs
            .filter(dsl::spotify_id.eq(spotify_id))
            .order(dsl::score.desc())
            .limit(1)
            .load_async::<YoutubeSong>(&self.pool)
            .await?;

        Ok(songs.into_iter().next())
    }

    /// Deletes a song from the database
    pub async fn delete_song(&self, id: i64) -> DatabaseResult<()> {
        use youtube_songs::dsl;
        delete(dsl::youtube_songs)
            .filter(dsl::id.eq(id))
            .execute_async(&self.pool)
            .await?;

        Ok(())
    }
}
