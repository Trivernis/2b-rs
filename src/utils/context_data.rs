use std::collections::HashMap;
use std::sync::Arc;

use serenity::model::id::GuildId;
use serenity::prelude::TypeMapKey;
use tokio::sync::Mutex;

use crate::providers::music::queue::MusicQueue;
use crate::providers::music::spotify::SpotifyApi;
use crate::utils::messages::EventDrivenMessage;
use database::Database;
use serenity::client::Context;

pub struct Store;

pub struct StoreData {
    pub minecraft_data_api: minecraft_data_rs::api::Api,
    pub music_queues: HashMap<GuildId, Arc<Mutex<MusicQueue>>>,
    pub spotify_api: SpotifyApi,
}

impl StoreData {
    pub fn new() -> StoreData {
        Self {
            minecraft_data_api: minecraft_data_rs::api::Api::new(
                minecraft_data_rs::api::versions::latest_stable().unwrap(),
            ),
            music_queues: HashMap::new(),
            spotify_api: SpotifyApi::new(),
        }
    }
}

impl TypeMapKey for Store {
    type Value = StoreData;
}

pub struct DatabaseContainer;

impl TypeMapKey for DatabaseContainer {
    type Value = Database;
}

/// Returns a copy of the database
pub async fn get_database_from_context(ctx: &Context) -> Database {
    let data = ctx.data.read().await;
    let database = data
        .get::<DatabaseContainer>()
        .expect("Invalid Context setup: Missing database");

    database.clone()
}

pub struct EventDrivenMessageContainer;

impl TypeMapKey for EventDrivenMessageContainer {
    type Value = HashMap<(u64, u64), Box<dyn EventDrivenMessage>>;
}
