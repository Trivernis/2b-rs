use crate::client::get_client;
use crate::utils::logging::init_logger;

pub mod client;
mod commands;
pub mod handler;
mod providers;
pub mod utils;

#[tokio::main]
async fn main() {
    init_logger();
    let mut client = get_client().await.unwrap();

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        log::error!("An error occurred while running the client: {:?}", why);
    }
}
