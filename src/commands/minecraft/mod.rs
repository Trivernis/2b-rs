use serenity::framework::standard::macros::group;

use enchantment::ENCHANTMENT_COMMAND;
use item::ITEM_COMMAND;

mod enchantment;
mod item;

#[group]
#[commands(item, enchantment)]
#[prefixes("mc", "minecraft")]
pub(crate) struct Minecraft;
