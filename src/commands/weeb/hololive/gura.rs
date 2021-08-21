use crate::commands::weeb::post_random_media;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

#[command]
#[description("Posts a random gura")]
#[usage("")]
#[aliases("a", "shark", "city-pop-shark", "same")]
#[bucket("general")]
async fn gura(ctx: &Context, msg: &Message) -> CommandResult {
    post_random_media(ctx, msg, "gura").await
}
