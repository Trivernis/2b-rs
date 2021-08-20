use crate::messages::inspirobot::create_inspirobot_menu;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

#[command]
#[description("Get an inspiring quote")]
#[usage("")]
#[aliases("inspireme", "inspire-me", "inspiro")]
#[bucket("general")]
async fn inspirobot(ctx: &Context, msg: &Message) -> CommandResult {
    create_inspirobot_menu(ctx, msg.channel_id).await?;

    Ok(())
}
