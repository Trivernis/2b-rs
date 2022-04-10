use crate::utils::context_data::get_database_from_context;
use crate::utils::error::BotError;
use rand::prelude::IteratorRandom;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;

static CATEGORY_PREFIX: &str = "pain-";
static NOT_FOUND_PAIN: &str = "404";

#[command]
#[description("Various types of pain (pain-peko)")]
#[usage("<pain-type>")]
#[example("peko")]
#[min_args(1)]
#[max_args(1)]
#[bucket("general")]
async fn pain(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    tracing::debug!("Got pain command");
    let pain_type = args.message().to_lowercase();
    let database = get_database_from_context(ctx).await;
    let mut media = database
        .get_media_by_category(format!("{}{}", CATEGORY_PREFIX, pain_type))
        .await?;

    if media.is_empty() {
        tracing::debug!("No media found for pain {}. Using 404", pain_type);
        media = database
            .get_media_by_category(format!("{}{}", CATEGORY_PREFIX, NOT_FOUND_PAIN))
            .await?;
    }

    let entry = media
        .into_iter()
        .choose(&mut rand::thread_rng())
        .ok_or(BotError::from("No gifs found."))?;
    tracing::trace!("Gif for pain is {:?}", entry);
    msg.reply(ctx, entry.url).await?;

    Ok(())
}
