use serenity::framework::standard::macros::group;

use about::ABOUT_COMMAND;
use add_media::ADD_MEDIA_COMMAND;
use clear::CLEAR_COMMAND;
use fuck::FUCK_COMMAND;
use media::MEDIA_COMMAND;
use pain::PAIN_COMMAND;
use ping::PING_COMMAND;
use qalc::QALC_COMMAND;
use shutdown::SHUTDOWN_COMMAND;
use stats::STATS_COMMAND;
use time::TIME_COMMAND;
use timezones::TIMEZONES_COMMAND;
use xkcd::XKCD_COMMAND;

mod about;
mod add_media;
mod clear;
mod fuck;
pub(crate) mod help;
mod media;
mod pain;
mod ping;
mod qalc;
mod shutdown;
mod stats;
mod time;
mod timezones;
mod xkcd;

#[group]
#[commands(
    ping, stats, shutdown, time, timezones, qalc, about, add_media, media, pain, clear, xkcd, fuck
)]
pub struct Misc;
