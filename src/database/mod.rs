use std::str::FromStr;
use std::sync::Arc;

use rusqlite::{params, Connection, NO_PARAMS};
use serenity::client::Context;
use serenity::model::id::GuildId;
use tokio::sync::Mutex;

use crate::database::guild::GuildSettings;
use crate::database::scripts::{CREATE_SCRIPT, UPDATE_SCRIPT};
use crate::utils::error::{BotError, BotResult};
use crate::utils::store::Store;
use std::fmt::Debug;

pub mod guild;
pub mod scripts;

#[derive(Debug)]
pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn new(connection: Connection) -> Self {
        Self { connection }
    }

    /// Initializes the database
    pub fn init(&self) -> BotResult<()> {
        self.connection.execute(CREATE_SCRIPT, NO_PARAMS)?;
        self.connection.execute(UPDATE_SCRIPT, NO_PARAMS)?;
        log::info!("Database initialized");

        Ok(())
    }

    /// Returns a guild setting
    pub fn get_guild_setting<T>(&self, guild_id: &GuildId, key: &str) -> BotResult<T>
    where
        T: Clone + FromStr + Debug,
    {
        log::trace!(
            "Fetching value of guild setting '{}' for guild {}",
            key,
            guild_id
        );
        self.connection
            .query_row(
                "SELECT guild_id, setting_key, setting_value FROM guild_settings WHERE guild_id = ?1 AND setting_key = ?2",
                params![guild_id.to_string(), key],
                |r| Ok(serde_rusqlite::from_row::<GuildSettings>(r).unwrap()),
            )
            .map_err(BotError::from)
            .and_then(|s| {
                s.setting_value
                    .parse::<T>()
                    .map_err(|_| BotError::from("Failed to parse Setting"))
            })
    }

    /// Sets a guild setting and overrides it if it already exists
    pub fn set_guild_setting<T>(&self, guild_id: &GuildId, key: &str, value: T) -> BotResult<()>
    where
        T: ToString + FromStr + Clone + Debug,
    {
        if self.get_guild_setting::<T>(guild_id, key).is_ok() {
            log::trace!("Clearing previous guild setting");
            self.connection.execute(
                "DELETE FROM guild_settings WHERE guild_id = ?1 AND setting_key = ?2",
                params![guild_id.to_string(), key],
            )?;
        }
        self.connection.execute(
            "INSERT INTO guild_settings (guild_id, setting_key, setting_value) VALUES (?1, ?2, ?3)",
            params![guild_id.to_string(), key, value.to_string()],
        )?;
        log::debug!(
            "Setting '{}' set to '{:?}' for guild {}",
            key,
            value,
            guild_id
        );

        Ok(())
    }
}

pub fn get_database() -> BotResult<Database> {
    let filename = dotenv::var("DB_NAME").unwrap_or("bot.db".to_string());
    let connection = rusqlite::Connection::open(filename)?;
    let database = Database::new(connection);
    database.init()?;

    Ok(database)
}

/// Returns a reference to a guilds music queue
pub(crate) async fn get_database_from_context(ctx: &Context) -> Arc<Mutex<Database>> {
    let data = ctx.data.read().await;
    let store = data.get::<Store>().unwrap();

    Arc::clone(&store.database)
}
