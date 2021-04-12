use crate::utils::context_data::get_database_from_context;
use bot_coreutils::url;
use bot_serenityutils::core::SHORT_TIMEOUT;
use bot_serenityutils::ephemeral_message::EphemeralMessage;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;

#[command]
#[description("Simple ping test command")]
#[usage("<url> (<category>) (<name>)")]
#[bucket("general")]
#[aliases("add-gif", "addgif")]
#[min_args(1)]
#[max_args(3)]
#[owners_only]
async fn add_gif(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let url = args.single::<String>()?;

    if !url::is_valid(&url) {
        msg.reply(ctx, "Invalid url").await?;
        return Ok(());
    }
    let category = args.single_quoted::<String>().ok();
    let name = args.single_quoted::<String>().ok();
    let database = get_database_from_context(&ctx).await;

    database.add_gif(&url, category, name).await?;
    EphemeralMessage::create(&ctx.http, msg.channel_id, SHORT_TIMEOUT, |c| {
        c.reference_message(msg)
            .content("Gif added to the database.")
    })
    .await?;

    Ok(())
}
