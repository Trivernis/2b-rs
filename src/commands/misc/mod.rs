use serenity::framework::standard::macros::group;

use about::ABOUT_COMMAND;
use ping::PING_COMMAND;
use qalc::QALC_COMMAND;
use shutdown::SHUTDOWN_COMMAND;
use stats::STATS_COMMAND;
use time::TIME_COMMAND;
use timezones::TIMEZONES_COMMAND;

mod about;
pub(crate) mod help;
mod ping;
mod qalc;
mod shutdown;
mod stats;
mod time;
mod timezones;

#[group]
#[commands(ping, stats, shutdown, time, timezones, qalc, about)]
pub struct Misc;
