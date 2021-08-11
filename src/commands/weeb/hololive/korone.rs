use crate::commands::weeb::post_random_media;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

#[command]
#[description("Posts a random korone gif")]
#[usage("")]
#[aliases("yubi")]
#[bucket("general")]
async fn korone(ctx: &Context, msg: &Message) -> CommandResult {
    post_random_media(ctx, msg, "korone").await
}
