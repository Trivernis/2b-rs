use serenity::framework::standard::macros::group;

mod matsuri;
mod pekofy;
mod sauce;

use matsuri::MATSURI_COMMAND;
use pekofy::PEKOFY_COMMAND;
use sauce::SAUCE_COMMAND;

#[group]
#[commands(pekofy, sauce, matsuri)]
pub struct Weeb;
