use crate::commands::weeb::post_random_media;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

#[command]
#[description("Posts a random nenechi")]
#[usage("")]
#[aliases(
    "nenechi", "suzumomo", "supernenechi",
    "super-hyper-ultra-ultimate-deluxe-perfect-amazing-shining-god-東方不敗-master-ginga-victory-strong-cute-beautiful-galaxy-baby-無限-無敵-無双-nenechi"
)]
#[bucket("general")]
async fn nene(ctx: &Context, msg: &Message) -> CommandResult {
    post_random_media(ctx, msg, "nene").await
}
