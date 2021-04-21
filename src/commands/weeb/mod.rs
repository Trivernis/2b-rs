use serenity::framework::standard::macros::group;

mod korone;
mod matsuri;
mod pekofy;
mod sauce;

use crate::utils::context_data::get_database_from_context;
use crate::utils::error::BotError;
use rand::prelude::IteratorRandom;
use serenity::client::Context;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

use korone::KORONE_COMMAND;
use matsuri::MATSURI_COMMAND;
use pekofy::PEKOFY_COMMAND;
use sauce::SAUCE_COMMAND;

#[group]
#[commands(pekofy, sauce, matsuri, korone)]
pub struct Weeb;

/// Posts a random media entry with the given category
async fn post_random_media(ctx: &Context, msg: &Message, category: &str) -> CommandResult {
    let database = get_database_from_context(ctx).await;
    let media = database.get_media_by_category(category).await?;
    let gif = media
        .into_iter()
        .choose(&mut rand::thread_rng())
        .ok_or(BotError::from("No media found."))?;

    msg.channel_id.say(ctx, gif.url).await?;

    Ok(())
}
