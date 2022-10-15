use crate::utils::context_data::get_database_from_context;
use bot_coreutils::url;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;
use serenity_additions::core::SHORT_TIMEOUT;
use serenity_additions::ephemeral_message::EphemeralMessage;

#[command]
#[description("Adds media to the database")]
#[usage("<url> [<category>] [<name>]")]
#[bucket("general")]
#[aliases("add_gif", "add-gif", "addgif", "add-media", "addmedia")]
#[min_args(1)]
#[max_args(3)]
#[owners_only]
async fn add_media(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let url = args.single::<String>()?;

    if !url::is_valid(&url) {
        msg.reply(ctx, "Invalid url").await?;
        return Ok(());
    }
    let category = args.single_quoted::<String>().ok();
    let name = args.single_quoted::<String>().ok();
    let database = get_database_from_context(&ctx).await;

    database.add_media(url, category, name).await?;
    EphemeralMessage::create(&ctx.http, msg.channel_id, SHORT_TIMEOUT, |c| {
        c.reference_message(msg)
            .content("Media entry added to the database.")
    })
    .await?;

    Ok(())
}
