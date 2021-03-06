use crate::commands::weeb::post_random_media;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

#[command]
#[description("Posts a random mumei gif")]
#[usage("")]
#[aliases("mumei", "forgowl", "owl", "nanashi")]
#[bucket("general")]
async fn mumei(ctx: &Context, msg: &Message) -> CommandResult {
    post_random_media(ctx, msg, "mumei").await
}
