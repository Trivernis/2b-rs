use serenity::framework::standard::macros::group;

use about::ABOUT_COMMAND;
use add_gif::ADD_GIF_COMMAND;
use gifs::GIFS_COMMAND;
use pain::PAIN_COMMAND;
use ping::PING_COMMAND;
use qalc::QALC_COMMAND;
use shutdown::SHUTDOWN_COMMAND;
use stats::STATS_COMMAND;
use time::TIME_COMMAND;
use timezones::TIMEZONES_COMMAND;

mod about;
mod add_gif;
mod gifs;
pub(crate) mod help;
mod pain;
mod ping;
mod qalc;
mod shutdown;
mod stats;
mod time;
mod timezones;

#[group]
#[commands(
    ping, stats, shutdown, time, timezones, qalc, about, add_gif, gifs, pain
)]
pub struct Misc;
