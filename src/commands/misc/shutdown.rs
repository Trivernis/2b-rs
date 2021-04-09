use crate::commands::common::handle_autodelete;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use serenity::prelude::*;
use std::process;

#[command]
#[description("Shutdown")]
#[usage("")]
#[owners_only]
async fn shutdown(ctx: &Context, msg: &Message) -> CommandResult {
    log::info!("Shutting down...");
    msg.channel_id
        .say(ctx, ":night_with_stars: Good night ....")
        .await?;
    handle_autodelete(ctx, msg).await?;
    process::exit(0);
}
