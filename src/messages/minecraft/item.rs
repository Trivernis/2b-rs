use crate::providers::minecraft::ItemFullInformation;
use crate::utils::error::BotResult;
use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::model::prelude::ChannelId;

pub async fn create_item_message(
    ctx: &Context,
    channel_id: ChannelId,
    item: ItemFullInformation,
) -> BotResult<Message> {
    let message = channel_id
        .send_message(ctx, |m| {
            m.embed(|mut e| {
                e = e
                    .title(&item.name)
                    .thumbnail(format!(
                        "https://minecraftitemids.com/item/128/{}.png",
                        item.id
                    ))
                    .field("Name", &item.name, false)
                    .field("Stack Size", item.stack_size, false);

                if let Some(durability) = item.durability {
                    e.field("Durability", durability, true);
                }

                if let Some(food) = &item.food {
                    e.field("Saturation", food.saturation, true);
                }

                if let Some(block) = &item.block {
                    e.field("Hardness", block.hardness.unwrap_or(0f32), true)
                        .field(
                            "Blast Resistance",
                            block.blast_resistance.unwrap_or(0f32),
                            true,
                        )
                        .field("Transparent", block.transparent, true)
                        .field("Emission Level", block.emit_light, true);
                }

                if !item.enchantments.is_empty() {
                    e.field("Enchantments", item.enchantments.join(", "), false);
                }

                e
            })
        })
        .await?;

    Ok(message)
}
