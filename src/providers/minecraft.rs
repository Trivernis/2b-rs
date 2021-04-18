use crate::utils::error::{BotError, BotResult};
use minecraft_data_rs::api::Api;
use minecraft_data_rs::models::block::Block;
use minecraft_data_rs::models::food::Food;

#[derive(Clone, Debug)]
pub struct ItemFullInformation {
    pub id: String,
    pub name: String,
    pub enchantments: Vec<String>,
    pub durability: Option<u32>,
    pub stack_size: u8,
    pub food: Option<Food>,
    pub block: Option<Block>,
}

pub fn get_item_full_information(name: &str, api: &Api) -> BotResult<ItemFullInformation> {
    let items_by_name = api.items.items_by_name()?;
    let item = items_by_name.get(name).ok_or(BotError::Msg(format!(
        "The item `{}` could not be found",
        name
    )))?;
    let enchantments_by_category = api.enchantments.enchantments_by_category()?;
    let mut enchantments = Vec::new();

    if let Some(enchant_categories) = &item.enchant_categories {
        enchantments = enchant_categories
            .into_iter()
            .filter_map(|c| enchantments_by_category.get(c))
            .flatten()
            .map(|e| e.display_name.clone())
            .collect::<Vec<String>>();
    }
    let food_by_name = api.foods.foods_by_name()?;
    let blocks_by_name = api.blocks.blocks_by_name()?;

    log::trace!("Item is {:?}", item);
    Ok(ItemFullInformation {
        id: item.name.clone(),
        name: item.display_name.clone(),
        enchantments,
        durability: item.durability.clone(),
        stack_size: item.stack_size,
        food: food_by_name.get(name).cloned(),
        block: blocks_by_name.get(name).cloned(),
    })
}
