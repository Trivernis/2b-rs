use crate::client::get_client;

pub(crate) mod client;
mod commands;
pub(crate) mod database;
mod providers;
pub(crate) mod utils;

#[tokio::main]
async fn main() {
    let mut client = get_client().await.unwrap();

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
