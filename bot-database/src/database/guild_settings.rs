use sea_orm::ActiveValue::Set;
use std::any;
use std::fmt::Debug;
use std::str::FromStr;

use crate::entity::guild_settings;
use crate::error::DatabaseResult;
use sea_orm::prelude::*;

impl super::BotDatabase {
    /// Returns a guild setting from the database
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn get_guild_setting<T: 'static, S: AsRef<str> + Debug>(
        &self,
        guild_id: u64,
        key: S,
    ) -> DatabaseResult<Option<T>>
    where
        T: FromStr,
    {
        let setting = guild_settings::Entity::find()
            .filter(guild_settings::Column::GuildId.eq(guild_id as i64))
            .filter(guild_settings::Column::Key.eq(key.as_ref()))
            .one(&self.db)
            .await?;
        if let Some(setting) = setting {
            if any::TypeId::of::<T>() == any::TypeId::of::<bool>() {
                Ok(setting
                    .value
                    .clone()
                    .unwrap_or("false".to_string())
                    .parse::<T>()
                    .ok())
            } else {
                Ok(setting.value.clone().and_then(|v| v.parse::<T>().ok()))
            }
        } else {
            Ok(None)
        }
    }

    /// Upserting a guild setting
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn set_guild_setting<T>(
        &self,
        guild_id: u64,
        key: String,
        value: T,
    ) -> DatabaseResult<()>
    where
        T: 'static + ToString + FromStr + Debug,
    {
        let model = guild_settings::ActiveModel {
            guild_id: Set(guild_id as i64),
            key: Set(key.clone()),
            value: Set(Some(value.to_string())),
            ..Default::default()
        };
        if self
            .get_guild_setting::<T, _>(guild_id, &key)
            .await?
            .is_some()
        {
            model.update(&self.db).await?;
        } else {
            model.insert(&self.db).await?;
        }

        Ok(())
    }

    /// Deletes a guild setting
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn delete_guild_setting<S: AsRef<str> + Debug>(
        &self,
        guild_id: u64,
        key: S,
    ) -> DatabaseResult<()> {
        guild_settings::Entity::delete_many()
            .filter(guild_settings::Column::GuildId.eq(guild_id))
            .filter(guild_settings::Column::Key.eq(key.as_ref()))
            .exec(&self.db)
            .await?;

        Ok(())
    }
}
