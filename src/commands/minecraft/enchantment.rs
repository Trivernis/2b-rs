use serenity::client::Context;
use serenity::framework::standard::{Args, CommandError, CommandResult, macros::command};
use serenity::model::channel::Message;

use crate::utils::store::Store;

#[command]
#[description("Provides information for a single enchantment")]
#[usage("enchantment <enchantment-name>")]
#[example("item unbreaking")]
#[min_args(1)]
pub(crate) async fn enchantment(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let store = data.get::<Store>().expect("Failed to get store");
    let enchantment_name = args.message().to_lowercase();
    let enchantments_by_name = store
        .minecraft_data_api
        .enchantments
        .enchantments_by_name()?;
    let enchantment = enchantments_by_name
        .get(&enchantment_name)
        .ok_or(CommandError::from(format!(
            "Enchantment {} not found",
            enchantment_name
        )))?
        .clone();

    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|mut e| {
                e = e
                    .title(enchantment.display_name)
                    .field("Name", enchantment.name, false)
                    .field("Category", enchantment.category, false);
                if !enchantment.exclude.is_empty() {
                    e = e.field("Incompatible With", enchantment.exclude.join(", "), false);
                }
                e.field("Max Level", enchantment.max_level, true)
                    .field("Weight", enchantment.weight, true)
                    .field(
                        "Min Cost",
                        format!(
                            "{} * level + {}",
                            enchantment.min_cost.a, enchantment.min_cost.b
                        ),
                        false,
                    )
                    .field(
                        "Max Cost",
                        format!(
                            "{} * level + {}",
                            enchantment.max_cost.a, enchantment.max_cost.b
                        ),
                        false,
                    )
                    .field("Tradeable", enchantment.tradeable, true)
                    .field("Discoverable", enchantment.discoverable, true)
                    .field("Treasure Only", enchantment.treasure_only, true)
            })
        })
        .await?;

    Ok(())
}
