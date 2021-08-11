use rand::prelude::IteratorRandom;
use serenity::client::Context;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

use hololive::fubuki::FUBUKI_COMMAND;
use hololive::gura::GURA_COMMAND;
use hololive::inanis::INANIS_COMMAND;
use hololive::korone::KORONE_COMMAND;
use hololive::matsuri::MATSURI_COMMAND;
use hololive::miko::MIKO_COMMAND;
use hololive::pekofy::PEKOFY_COMMAND;
use hololive::rushia::RUSHIA_COMMAND;
use hololive::watame::WATAME_COMMAND;
use sauce::SAUCE_COMMAND;
use theme::THEME_COMMAND;

use crate::utils::context_data::get_database_from_context;
use crate::utils::error::BotError;

mod hololive;
mod sauce;
mod theme;

#[group]
#[commands(
    pekofy, sauce, matsuri, korone, rushia, fubuki, miko, theme, watame, inanis, gura
)]
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
