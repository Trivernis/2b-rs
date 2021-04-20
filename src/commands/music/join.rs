use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;

use crate::commands::common::handle_autodelete;
use crate::commands::music::{get_channel_for_author, get_music_player_for_guild, is_dj};
use crate::providers::music::player::MusicPlayer;
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
            forward_error!(
                ctx,
                msg.channel_id,
                get_channel_for_author(&msg.author.id, &guild)
            )
        }
    } else {
        forward_error!(
            ctx,
            msg.channel_id,
            get_channel_for_author(&msg.author.id, &guild)
        )
    };
    if get_music_player_for_guild(ctx, guild.id).await.is_some() {
        EphemeralMessage::create(&ctx.http, msg.channel_id, SHORT_TIMEOUT, |m| {
            m.content("‼️ I'm already in a Voice Channel")
        })
        .await?;
        return Ok(());
    }
    log::debug!("Joining channel {} for guild {}", channel_id, guild.id);
    MusicPlayer::join(ctx, guild.id, channel_id).await?;
    EphemeralMessage::create(&ctx.http, msg.channel_id, SHORT_TIMEOUT, |m| {
        m.content("🎤 Joined the Voice Channel")
    })
    .await?;
    handle_autodelete(ctx, msg).await?;

    Ok(())
}
