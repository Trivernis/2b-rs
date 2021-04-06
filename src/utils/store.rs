use crate::database::Database;
use parking_lot::Mutex;
use serenity::prelude::TypeMapKey;
use std::sync::Arc;

pub struct Store;

pub struct StoreData {
    pub database: Arc<Mutex<Database>>,
    pub minecraft_data_api: minecraft_data_rs::api::Api,
}

impl StoreData {
    pub fn new(database: Database) -> StoreData {
        Self {
            database: Arc::new(Mutex::new(database)),
            minecraft_data_api: minecraft_data_rs::api::Api::new(
                minecraft_data_rs::api::versions::latest_stable().unwrap(),
            ),
        }
    }
}

impl TypeMapKey for Store {
    type Value = StoreData;
}
