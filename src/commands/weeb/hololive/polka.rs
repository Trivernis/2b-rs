use crate::commands::weeb::post_random_media;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

#[command]
#[description("Posts a random pol")]
#[usage("")]
#[aliases("pol", "omarun", "omapol")]
#[bucket("general")]
async fn polka(ctx: &Context, msg: &Message) -> CommandResult {
    post_random_media(ctx, msg, "polka").await
}
