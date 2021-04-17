use std::process;

use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::commands::common::handle_autodelete;

#[command]
#[description("Shuts down the bot with the specified exit code")]
#[min_args(0)]
#[max_args(1)]
#[usage("[<code>]")]
#[owners_only]
async fn shutdown(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let code = args.single::<i32>().unwrap_or(0);
    log::info!("Shutting down with code {}...", code);
    msg.channel_id
        .say(
            ctx,
            format!(":night_with_stars: Good night (code: {})...", code),
        )
        .await?;
    handle_autodelete(ctx, msg).await?;
    process::exit(code);
}
