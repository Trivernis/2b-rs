use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;

use crate::commands::common::handle_autodelete;
use crate::commands::music::{get_channel_for_author, is_dj, join_channel};
use bot_serenityutils::core::SHORT_TIMEOUT;
use bot_serenityutils::ephemeral_message::EphemeralMessage;
use serenity::model::id::ChannelId;

#[command]
#[only_in(guilds)]
#[description("Joins a voice channel")]
#[usage("")]
#[bucket("general")]
async fn join(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let channel_id = if let Ok(arg) = args.single::<u64>() {
        if is_dj(ctx, guild.id, &msg.author).await? {
            ChannelId(arg)
        } else {
            get_channel_for_author(&msg.author.id, &guild)?
        }
    } else {
        get_channel_for_author(&msg.author.id, &guild)?
    };
    log::debug!("Joining channel {} for guild {}", channel_id, guild.id);
    join_channel(ctx, channel_id, guild.id).await;
    EphemeralMessage::create(&ctx.http, msg.channel_id, SHORT_TIMEOUT, |m| {
        m.content("🎤 Joined the Voice Channel")
    })
    .await?;
    handle_autodelete(ctx, msg).await?;

    Ok(())
}
