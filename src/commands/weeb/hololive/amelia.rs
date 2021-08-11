use crate::commands::weeb::post_random_media;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

#[command]
#[description("Posts a random amelia")]
#[usage("")]
#[aliases("ame", "ground-pounder")]
#[bucket("general")]
async fn amelia(ctx: &Context, msg: &Message) -> CommandResult {
    post_random_media(ctx, msg, "amelia").await
}
