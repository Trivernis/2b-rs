use serenity::framework::standard::macros::group;

use ping::PING_COMMAND;
use shutdown::SHUTDOWN_COMMAND;
use stats::STATS_COMMAND;

pub(crate) mod help;
mod ping;
mod shutdown;
mod stats;

#[group]
#[commands(ping, stats, shutdown)]
pub struct Misc;
