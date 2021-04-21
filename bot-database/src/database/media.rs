use diesel::insert_into;
use diesel::prelude::*;
use tokio_diesel::*;

use crate::error::DatabaseResult;
use crate::models::*;
use crate::schema::*;
use crate::Database;

impl Database {
    /// Returns a list of all gifs in the database
    pub async fn get_all_media(&self) -> DatabaseResult<Vec<Media>> {
        use media::dsl;
        log::debug!("Loading all gifs from the database");

        let gifs: Vec<Media> = dsl::media.load_async::<Media>(&self.pool).await?;
        Ok(gifs)
    }

    /// Returns a list of gifs by assigned category
    pub async fn get_media_by_category(&self, category: &str) -> DatabaseResult<Vec<Media>> {
        use media::dsl;
        log::debug!("Searching for gifs in category '{}'", category);

        let gifs: Vec<Media> = dsl::media
            .filter(dsl::category.eq(category))
            .load_async::<Media>(&self.pool)
            .await?;
        Ok(gifs)
    }

    /// Adds a gif to the database
    pub async fn add_media(
        &self,
        url: &str,
        category: Option<String>,
        name: Option<String>,
    ) -> DatabaseResult<()> {
        use media::dsl;
        log::debug!(
            "Inserting gif with url '{}' and name {:?} and category {:?}",
            url,
            name,
            category
        );
        insert_into(dsl::media)
            .values(MediaInsert {
                url: url.to_string(),
                name,
                category,
            })
            .execute_async(&self.pool)
            .await?;

        Ok(())
    }
}
