use crate::schema::*;

#[derive(Queryable, Debug)]
pub struct GuildSetting {
    pub guild_id: i64,
    pub key: String,
    pub value: Option<String>,
}

#[derive(Insertable, Debug)]
#[table_name = "guild_settings"]
pub struct GuildSettingInsert {
    pub guild_id: i64,
    pub key: String,
    pub value: String,
}

#[derive(Queryable, Debug)]
pub struct GuildPlaylist {
    pub guild_id: i64,
    pub name: String,
    pub url: String,
}

#[derive(Insertable, Debug)]
#[table_name = "guild_playlists"]
pub struct GuildPlaylistInsert {
    pub guild_id: i64,
    pub name: String,
    pub url: String,
}