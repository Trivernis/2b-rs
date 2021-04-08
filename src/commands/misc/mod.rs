use serenity::framework::standard::macros::group;

use pekofy::PEKOFY_COMMAND;
use ping::PING_COMMAND;
use shutdown::SHUTDOWN_COMMAND;
use stats::STATS_COMMAND;

pub(crate) mod help;
mod pekofy;
mod ping;
mod shutdown;
mod stats;

#[group]
#[commands(ping, stats, shutdown, pekofy)]
pub struct Misc;
