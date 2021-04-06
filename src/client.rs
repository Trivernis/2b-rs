use crate::database::{get_database, Database};
use crate::utils::error::{BotError, BotResult};
use crate::utils::store::{Store, StoreData};
use serenity::async_trait;
use serenity::client::EventHandler;
use serenity::framework::StandardFramework;
use serenity::Client;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

pub async fn get_client() -> BotResult<Client> {
    let token = dotenv::var("BOT_TOKEN").map_err(|_| BotError::MissingToken)?;
    let database = get_database()?;

    let client = Client::builder(token).framework(get_framework()).await?;
    {
        let mut data = client.data.write().await;
        data.insert::<Store>(StoreData::new(database))
    }

    Ok(client)
}

pub fn get_framework() -> StandardFramework {
    StandardFramework::default().configure(|c| {
        c.prefix(
            dotenv::var("BOT_PREFIX")
                .unwrap_or("~!".to_string())
                .as_str(),
        )
        .allow_dm(true)
        .ignore_bots(true)
    })
}
