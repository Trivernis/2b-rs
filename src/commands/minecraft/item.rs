use serenity::client::Context;
use serenity::framework::standard::{macros::command, Args, CommandError, CommandResult};
use serenity::model::channel::Message;

use crate::utils::context_data::Store;

#[command]
#[description("Provides information for a single minecraft item")]
#[usage("<item-name>")]
#[example("bread")]
#[min_args(1)]
#[aliases("i")]
pub(crate) async fn item(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let store = data.get::<Store>().expect("Failed to get store");

    let item_name = args.message().to_lowercase();
    log::debug!("Searching for item '{}'", item_name);
    let items_by_name = store.minecraft_data_api.items.items_by_name()?;
    let item = items_by_name
        .get(&item_name)
        .ok_or(CommandError::from(format!(
            "The item `{}` could not be found",
            item_name
        )))?;
    let enchantments_by_category = store
        .minecraft_data_api
        .enchantments
        .enchantments_by_category()?;
    log::trace!("Item is {:?}", item);

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|mut e| {
                e = e
                    .title(&*item.display_name)
                    .thumbnail(format!(
                        "https://minecraftitemids.com/item/128/{}.png",
                        item.name
                    ))
                    .field("Name", &*item.name, false)
                    .field("Stack Size", item.stack_size, false);
                if let Some(durability) = item.durability {
                    e = e.field("Durability", durability, true);
                }
                if let Some(variations) = &item.variations {
                    e = e.field("Variations", format!("{:?}", variations), false);
                }
                if let Some(enchant_categories) = &item.enchant_categories {
                    let item_enchantments = enchant_categories
                        .into_iter()
                        .filter_map(|c| enchantments_by_category.get(c))
                        .flatten()
                        .map(|e| e.display_name.clone())
                        .collect::<Vec<String>>();
                    e = e.field("Enchantments", item_enchantments.join(", "), false);
                }

                e
            })
        })
        .await?;

    Ok(())
}
