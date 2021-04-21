use crate::commands::weeb::post_random_media;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

#[command]
#[description("Posts a random miko")]
#[usage("")]
#[aliases("faq", "elite")]
#[bucket("general")]
async fn miko(ctx: &Context, msg: &Message) -> CommandResult {
    post_random_media(ctx, msg, "miko").await
}
