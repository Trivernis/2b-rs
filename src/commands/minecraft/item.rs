use serenity::client::Context;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::channel::Message;

use crate::commands::common::handle_autodelete;
use crate::messages::minecraft::item::create_item_message;
use crate::providers::minecraft::get_item_full_information;
use crate::utils::context_data::Store;

#[command]
#[description("Provides information for a single minecraft item")]
#[usage("<item-name>")]
#[example("bread")]
#[min_args(1)]
#[aliases("i")]
#[bucket("general")]
pub(crate) async fn item(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let store = data.get::<Store>().expect("Failed to get store");

    let item_name = args.message().to_lowercase();
    tracing::debug!("Searching for item '{}'", item_name);
    let information = get_item_full_information(&item_name, &store.minecraft_data_api)?;
    tracing::trace!("Item full information is {:?}", information);
    create_item_message(ctx, msg.channel_id, information).await?;

    handle_autodelete(ctx, msg).await?;

    Ok(())
}
