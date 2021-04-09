use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

use crate::commands::common::handle_autodelete;
use crate::commands::music::{get_channel_for_author, join_channel};

#[command]
#[only_in(guilds)]
#[description("Joins a voice channel")]
#[usage("")]
async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let channel_id = get_channel_for_author(&msg.author.id, &guild)?;
    log::debug!("Joining channel {} for guild {}", channel_id, guild.id);
    join_channel(ctx, channel_id, guild.id).await;
    handle_autodelete(ctx, msg).await?;

    Ok(())
}
