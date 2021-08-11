use crate::commands::weeb::post_random_media;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

#[command]
#[description("Posts a random rushia")]
#[usage("")]
#[aliases("wataoji", "wata-oji", "watamelon")]
#[bucket("general")]
async fn watame(ctx: &Context, msg: &Message) -> CommandResult {
    post_random_media(ctx, msg, "watame").await
}
