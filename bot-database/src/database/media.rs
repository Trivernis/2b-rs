use crate::entity::media;
use crate::error::DatabaseResult;
use sea_orm::prelude::*;
use sea_orm::ActiveValue::Set;
use std::fmt::Debug;

impl super::BotDatabase {
    /// Returns a list of all gifs in the database
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn get_all_media(&self) -> DatabaseResult<Vec<media::Model>> {
        let entries = media::Entity::find().all(&self.db).await?;

        Ok(entries)
    }

    /// Returns a list of gifs by assigned category
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn get_media_by_category<S: AsRef<str> + 'static + Debug>(
        &self,
        category: S,
    ) -> DatabaseResult<Vec<media::Model>> {
        let entries = media::Entity::find()
            .filter(media::Column::Category.eq(category.as_ref()))
            .all(&self.db)
            .await?;

        Ok(entries)
    }

    /// Adds a gif to the database
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn add_media(
        &self,
        url: String,
        category: Option<String>,
        name: Option<String>,
    ) -> DatabaseResult<()> {
        let model = media::ActiveModel {
            url: Set(url),
            category: Set(category),
            name: Set(name),
            ..Default::default()
        };
        model.insert(&self.db).await?;

        Ok(())
    }
}
