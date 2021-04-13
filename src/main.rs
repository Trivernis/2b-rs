use crate::client::get_client;
use crate::utils::logging::init_logger;

#[macro_use]
extern crate bot_serenityutils;

pub mod client;
mod commands;
pub mod handler;
mod messages;
mod providers;
pub mod utils;

pub static VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
    init_logger();
    let mut client = get_client().await.unwrap();

    // start listening for events by starting a single shard
    if let Err(why) = client.start_autosharded().await {
        log::error!("An error occurred while running the client: {:?}", why);
    }
}
