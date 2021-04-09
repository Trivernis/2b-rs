use serenity::framework::standard::macros::group;

use pekofy::PEKOFY_COMMAND;
use ping::PING_COMMAND;
use shutdown::SHUTDOWN_COMMAND;
use stats::STATS_COMMAND;
use time::TIME_COMMAND;
use timezones::TIMEZONES_COMMAND;

pub(crate) mod help;
mod pekofy;
mod ping;
mod shutdown;
mod stats;
mod time;
mod timezones;

#[group]
#[commands(ping, stats, shutdown, pekofy, time, timezones)]
pub struct Misc;
