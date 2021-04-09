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
