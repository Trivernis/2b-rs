mod enchantment;
mod item;

use enchantment::ENCHANTMENT_COMMAND;
use item::ITEM_COMMAND;
use serenity::framework::standard::macros::group;

#[group]
#[commands(item, enchantment)]
#[prefix("mc")]
pub(crate) struct Minecraft;
