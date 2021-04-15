use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::commands::common::handle_autodelete;
use crate::commands::music::{get_queue_for_guild, is_dj};
use crate::messages::music::now_playing::update_now_playing_msg;
use bot_serenityutils::core::SHORT_TIMEOUT;
use bot_serenityutils::ephemeral_message::EphemeralMessage;

#[command]
#[only_in(guilds)]
#[description("Pauses playback")]
#[usage("")]
#[bucket("general")]
async fn pause(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    log::debug!("Pausing playback for guild {}", guild.id);
    if !is_dj(ctx, guild.id, &msg.author).await? {
        msg.channel_id.say(ctx, "Requires DJ permissions").await?;
        return Ok(());
    }

    let queue = forward_error!(
        ctx,
        msg.channel_id,
        get_queue_for_guild(ctx, &guild.id).await
    );
    let mut queue_lock = queue.lock().await;

    if let Some(_) = queue_lock.current() {
        queue_lock.pause();
        if queue_lock.paused() {
            log::debug!("Paused");
            EphemeralMessage::create(&ctx.http, msg.channel_id, SHORT_TIMEOUT, |m| {
                m.content("⏸️ Paused playback️")
            })
            .await?;
            if let (Some(menu), Some((current, _))) =
                (&queue_lock.now_playing_msg, queue_lock.current())
            {
                update_now_playing_msg(&ctx.http, menu, current.metadata(), true).await?;
            }
        } else {
            log::debug!("Resumed");
            EphemeralMessage::create(&ctx.http, msg.channel_id, SHORT_TIMEOUT, |m| {
                m.content("▶ Resumed playback️")
            })
            .await?;
            if let (Some(menu), Some((current, _))) =
                (&queue_lock.now_playing_msg, queue_lock.current())
            {
                update_now_playing_msg(&ctx.http, menu, current.metadata(), true).await?;
            }
        }
    } else {
        msg.channel_id.say(ctx, "Nothing to pause").await?;
    }
    handle_autodelete(ctx, msg).await?;

    Ok(())
}
