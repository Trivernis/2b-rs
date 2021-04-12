use crate::messages::gifs::create_gifs_menu;
use crate::utils::context_data::get_database_from_context;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

#[command]
#[description("Displays a list of all gifs used by the bot")]
#[bucket("general")]
async fn gifs(ctx: &Context, msg: &Message) -> CommandResult {
    let database = get_database_from_context(ctx).await;
    let gifs = database.get_all_gifs().await?;
    create_gifs_menu(ctx, msg.channel_id, gifs).await?;

    Ok(())
}
