use serenity::framework::standard::macros::group;

use ping::PING_COMMAND;
use stats::STATS_COMMAND;

pub(crate) mod help;
mod ping;
mod stats;

#[group]
#[commands(ping, stats)]
pub struct Misc;
