use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Guild {
    guild_id: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct GuildSettings {
    pub guild_id: String,
    pub setting_key: String,
    pub setting_value: String,
}

pub static SETTING_AUTOSHUFFLE: &str = "music.autoshuffle";
pub static GUILD_SETTINGS: &[&str] = &[SETTING_AUTOSHUFFLE];
