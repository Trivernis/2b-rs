use serenity::framework::standard::macros::command;
use serenity::framework::standard::{CommandError, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::commands::common::handle_autodelete;
use crate::commands::music::{get_music_player_for_guild, DJ_CHECK};
use crate::messages::music::no_voicechannel::create_no_voicechannel_message;
use serenity_rich_interaction::core::SHORT_TIMEOUT;
use serenity_rich_interaction::ephemeral_message::EphemeralMessage;

#[command]
#[only_in(guilds)]
#[description("Pauses playback")]
#[usage("")]
#[bucket("general")]
#[checks(DJ)]
async fn pause(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    tracing::debug!("Pausing playback for guild {}", guild.id);

    let player = if let Some(player) = get_music_player_for_guild(ctx, guild.id).await {
        player
    } else {
        return create_no_voicechannel_message(&ctx.http, msg.channel_id)
            .await
            .map_err(CommandError::from);
    };
    let mut player = player.lock().await;

    if let Some(_) = player.queue().current() {
        player.toggle_paused().await?;
        let is_paused = player.is_paused();

        if is_paused {
            tracing::debug!("Paused");
            EphemeralMessage::create(&ctx.http, msg.channel_id, SHORT_TIMEOUT, |m| {
                m.content("⏸️ Paused playback️")
            })
            .await?;
            player.update_now_playing().await?;
        } else {
            tracing::debug!("Resumed");
            EphemeralMessage::create(&ctx.http, msg.channel_id, SHORT_TIMEOUT, |m| {
                m.content("▶ Resumed playback️")
            })
            .await?;
            player.update_now_playing().await?;
        }
    } else {
        msg.channel_id.say(ctx, "Nothing to pause").await?;
    }
    handle_autodelete(ctx, msg).await?;

    Ok(())
}
