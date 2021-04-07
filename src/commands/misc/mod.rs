pub(crate) mod ping;

use ping::PING_COMMAND;
use serenity::framework::standard::macros::group;

#[group]
#[commands(ping)]
pub struct Misc;
