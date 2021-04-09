use serenity::framework::standard::macros::group;

use get::GET_COMMAND;
use set::SET_COMMAND;

mod get;
mod set;

#[group]
#[commands(set, get)]
#[prefix("settings")]
pub struct Settings;

pub const SETTING_AUTOSHUFFLE: &str = "music.autoshuffle";
pub const GUILD_SETTINGS: &[&str] = &[SETTING_AUTOSHUFFLE];
