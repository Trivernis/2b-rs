use serenity::framework::standard::macros::group;

use ping::PING_COMMAND;

pub(crate) mod help;
mod ping;

#[group]
#[commands(ping)]
pub struct Misc;
