use serenity::framework::standard::macros::group;

use about::ABOUT_COMMAND;
use pekofy::PEKOFY_COMMAND;
use ping::PING_COMMAND;
use qalc::QALC_COMMAND;
use sauce::SAUCE_COMMAND;
use shutdown::SHUTDOWN_COMMAND;
use stats::STATS_COMMAND;
use time::TIME_COMMAND;
use timezones::TIMEZONES_COMMAND;

mod about;
pub(crate) mod help;
mod pekofy;
mod ping;
mod qalc;
mod sauce;
mod shutdown;
mod stats;
mod time;
mod timezones;

#[group]
#[commands(ping, stats, shutdown, pekofy, time, timezones, qalc, sauce, about)]
pub struct Misc;
