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
    process::exit(0);
}
