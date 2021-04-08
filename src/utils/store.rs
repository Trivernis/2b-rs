use std::collections::HashMap;
use std::sync::Arc;

use serenity::model::id::GuildId;
use serenity::prelude::TypeMapKey;
use tokio::sync::Mutex;

use crate::database::Database;
use crate::providers::music::queue::MusicQueue;

pub struct Store;

pub struct StoreData {
    pub database: Arc<Mutex<Database>>,
    pub minecraft_data_api: minecraft_data_rs::api::Api,
    pub music_queues: HashMap<GuildId, Arc<Mutex<MusicQueue>>>,
}

impl StoreData {
    pub fn new(database: Database) -> StoreData {
        Self {
            database: Arc::new(Mutex::new(database)),
            minecraft_data_api: minecraft_data_rs::api::Api::new(
                minecraft_data_rs::api::versions::latest_stable().unwrap(),
            ),
            music_queues: HashMap::new(),
        }
    }
}

impl TypeMapKey for Store {
    type Value = StoreData;
}
