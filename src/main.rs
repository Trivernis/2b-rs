use crate::client::get_client;
use crate::utils::logging::init_logger;

mod client;
mod commands;
mod handler;
mod messages;
mod providers;
#[macro_use]
mod utils;

pub static VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
    let _ = dotenv::dotenv();
    init_logger();
    let mut client = get_client()
        .await
        .map_err(|e| log::error!("Failed to get client: {:?}", e))
        .expect("Failed to get client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start_autosharded().await {
        log::error!("An error occurred while running the client: {:?}", why);
    }
}
