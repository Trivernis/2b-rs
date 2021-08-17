use crate::schema::*;
use std::time::SystemTime;

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

#[derive(Queryable, Debug, Clone)]
pub struct Media {
    pub id: i64,
    pub category: Option<String>,
    pub name: Option<String>,
    pub url: String,
}

#[derive(Insertable, Debug)]
#[table_name = "media"]
pub struct MediaInsert {
    pub category: Option<String>,
    pub name: Option<String>,
    pub url: String,
}

#[derive(Insertable, Debug)]
#[table_name = "statistics"]
pub struct StatisticInsert {
    pub version: String,
    pub command: String,
    pub executed_at: SystemTime,
    pub success: bool,
    pub error_msg: Option<String>,
}

#[derive(Queryable, Debug, Clone)]
pub struct YoutubeSong {
    pub id: i64,
    pub spotify_id: String,
    pub artist: String,
    pub title: String,
    pub album: String,
    pub url: String,
    pub score: i32,
}

#[derive(Insertable, Debug)]
#[table_name = "youtube_songs"]
pub struct YoutubeSongInsert {
    pub spotify_id: String,
    pub artist: String,
    pub title: String,
    pub album: String,
    pub url: String,
}

#[derive(Queryable, Debug, Clone)]
pub struct EphemeralMessage {
    pub channel_id: i64,
    pub message_id: i64,
    pub timeout: SystemTime,
}

#[derive(Insertable, Debug)]
#[table_name = "ephemeral_messages"]
pub struct EphemeralMessageInsert {
    pub channel_id: i64,
    pub message_id: i64,
    pub timeout: SystemTime,
}

#[derive(Queryable, Debug, Clone)]
pub struct Event {
    pub id: i32,
    pub guild_id: i64,
    pub channel_id: i64,
    pub name: String,
    pub description: String,
    pub event_start: SystemTime,
    pub event_end: Option<SystemTime>,
}

#[derive(Insertable, Debug)]
#[table_name = "events"]
pub struct EventInsert {
    pub guild_id: i64,
    pub channel_id: i64,
    pub name: String,
    pub description: String,
    pub event_start: SystemTime,
    pub event_end: Option<SystemTime>,
}
