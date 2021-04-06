use crate::database::Database;
use parking_lot::Mutex;
use serenity::prelude::TypeMapKey;
use std::sync::Arc;

pub struct Store;

pub struct StoreData {
    pub database: Arc<Mutex<Database>>,
}

impl StoreData {
    pub fn new(database: Database) -> StoreData {
        Self {
            database: Arc::new(Mutex::new(database)),
        }
    }
}

impl TypeMapKey for Store {
    type Value = StoreData;
}
