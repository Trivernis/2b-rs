use crate::utils::context_data::DatabaseContainer;
use crate::utils::error::{BotError, BotResult};
use serenity::client::Context;
use serenity::model::prelude::GuildId;
use std::str::FromStr;

pub static ALL_SETTINGS: &[Setting] = &[
    Setting::MusicAutoShuffle,
    Setting::BotAutoDelete,
    Setting::MusicDjRole,
];

#[derive(Clone, Debug)]
pub enum Setting {
    MusicAutoShuffle,
    MusicDjRole,
    BotAutoDelete,
}

impl ToString for Setting {
    fn to_string(&self) -> String {
        match self {
            Self::MusicAutoShuffle => "music.autoshuffle".to_string(),
            Self::BotAutoDelete => "bot.autodelete".to_string(),
            Self::MusicDjRole => "music.dj-role".to_string(),
        }
    }
}

/// Returns a specific guild setting
pub async fn get_setting<T: 'static + FromStr>(
    ctx: &Context,
    guild_id: GuildId,
    setting: Setting,
) -> BotResult<Option<T>> {
    let data = ctx.data.read().await;
    let database = data.get::<DatabaseContainer>().unwrap();
    database
        .get_guild_setting::<T>(guild_id.0, setting.to_string())
        .await
        .map_err(BotError::from)
}
