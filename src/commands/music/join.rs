use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

use crate::commands::music::{get_channel_for_author, join_channel};

#[command]
#[only_in(guilds)]
#[description("Joins a voice channel")]
#[usage("join")]
async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let channel_id = get_channel_for_author(&msg.author.id, &guild)?;
    join_channel(ctx, channel_id, guild.id).await;

    Ok(())
}
