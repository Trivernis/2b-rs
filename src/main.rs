use crate::client::get_client;
use serenity::client::{Context, EventHandler};
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult,
};
use serenity::model::channel::Message;

pub(crate) mod client;
pub(crate) mod database;
pub(crate) mod utils;

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[tokio::main]
async fn main() {
    let mut client = get_client().await.unwrap();

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}
