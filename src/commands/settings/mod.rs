use serenity::framework::standard::macros::group;

use get::GET_COMMAND;
use set::SET_COMMAND;

mod get;
mod set;

#[group]
#[commands(set, get)]
#[prefix("settings")]
pub struct Settings;
