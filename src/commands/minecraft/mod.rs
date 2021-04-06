use serenity::model::channel::Message;

use crate::utils::store::{Store, StoreData};
use serenity::client::Context;
use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandError, CommandResult,
};

#[group]
#[commands(durability)]
pub(crate) struct Minecraft;

#[command]
pub(crate) async fn durability(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        return Err(CommandError::from("You need to provide an item name"));
    }
    let data = ctx.data.read().await;
    let store = data.get::<Store>().expect("Failed to get store");

    let item_name = args.message().to_lowercase();
    let items_by_name = store.minecraft_data_api.items.items_by_name()?;
    let item = items_by_name
        .get(&item_name)
        .ok_or(CommandError::from(format!(
            "The item `{}` could not be found",
            item_name
        )))?;

    msg.channel_id
        .say(
            ctx,
            format!(
                "The durability for `{}` is `{:?}`",
                item.display_name, item.durability
            ),
        )
        .await?;

    Ok(())
}
