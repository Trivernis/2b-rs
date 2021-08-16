use crate::utils::context_data::get_database_from_context;
use crate::utils::error::BotError;
use bot_database::models::Media;
use rand::prelude::SliceRandom;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;

#[command]
#[description("Party command")]
#[max_args(1)]
#[usage("(<amount>)")]
#[bucket("general")]
async fn party(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut amount = args.single::<u32>().unwrap_or(1);
    if amount > 5 {
        amount = 5;
    }
    if amount == 0 {
        return Ok(());
    }
    let database = get_database_from_context(ctx).await;
    let mut media = database.get_media_by_category("party").await?;
    media.shuffle(&mut rand::thread_rng());
    let mut chosen_gifs = Vec::new();

    for _ in 0..amount {
        chosen_gifs.push(media.pop());
    }
    let chosen_gifs: Vec<Media> = chosen_gifs.into_iter().filter_map(|g| g).collect();
    if chosen_gifs.is_empty() {
        return Err(BotError::from("No media found.").into());
    }

    for gif in chosen_gifs {
        msg.channel_id
            .send_message(&ctx.http, |m| m.content(gif.url))
            .await?;
    }

    Ok(())
}
