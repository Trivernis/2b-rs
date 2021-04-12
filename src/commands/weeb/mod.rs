use serenity::framework::standard::macros::group;

mod pekofy;
mod sauce;

use pekofy::PEKOFY_COMMAND;
use sauce::SAUCE_COMMAND;

#[group]
#[commands(pekofy, sauce)]
pub struct Weeb;
