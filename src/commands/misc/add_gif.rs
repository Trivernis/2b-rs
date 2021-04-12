use crate::utils::context_data::get_database_from_context;
use bot_coreutils::url;
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
    msg.reply(ctx, "Gif added to database").await?;

    Ok(())
}
