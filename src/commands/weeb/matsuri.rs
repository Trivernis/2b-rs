use crate::utils::context_data::get_database_from_context;
use crate::utils::error::BotError;
use rand::prelude::IteratorRandom;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

static MEDIA_CATEGORY: &str = "matsuri";

#[command]
#[description("Posts a random matsuri gif")]
#[usage("")]
#[bucket("general")]
async fn matsuri(ctx: &Context, msg: &Message) -> CommandResult {
    let database = get_database_from_context(ctx).await;
    let media = database.get_media_by_category(MEDIA_CATEGORY).await?;
    let gif = media
        .into_iter()
        .choose(&mut rand::thread_rng())
        .ok_or(BotError::from("No gifs found."))?;

    msg.channel_id.say(ctx, gif.url).await?;

    Ok(())
}
