use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Guild {
    guild_id: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct GuildSettings {
    guild_id: i32,
    key: String,
    value: String,
}
