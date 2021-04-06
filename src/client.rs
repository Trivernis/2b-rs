use crate::commands::minecraft::Minecraft;
use crate::database::{get_database, Database};
use crate::utils::error::{BotError, BotResult};
use crate::utils::store::{Store, StoreData};
use serenity::async_trait;
use serenity::client::{Context, EventHandler};
use serenity::framework::standard::macros::hook;
use serenity::framework::standard::CommandResult;
use serenity::framework::StandardFramework;
use serenity::model::channel::Message;
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
    StandardFramework::default()
        .configure(|c| {
            c.prefix(
                dotenv::var("BOT_PREFIX")
                    .unwrap_or("~!".to_string())
                    .as_str(),
            )
            .allow_dm(true)
            .ignore_bots(true)
        })
        .group(&crate::commands::minecraft::MINECRAFT_GROUP)
        .group(&crate::GENERAL_GROUP)
        .after(after_hook)
}

#[hook]
async fn after_hook(ctx: &Context, msg: &Message, cmd_name: &str, error: CommandResult) {
    //  Print out an error if it happened
    if let Err(why) = error {
        let _ = msg.channel_id.say(&ctx, format!("{}", why)).await;
        println!("Error in {}: {:?}", cmd_name, why);
    }
}
