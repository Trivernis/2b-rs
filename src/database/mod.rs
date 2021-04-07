use rusqlite::{Connection, NO_PARAMS};

use crate::database::scripts::{CREATE_SCRIPT, UPDATE_SCRIPT};
use crate::utils::error::BotResult;

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
