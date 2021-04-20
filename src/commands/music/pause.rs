use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::commands::common::handle_autodelete;
use crate::commands::music::{get_queue_for_guild, DJ_CHECK};
use crate::messages::music::now_playing::update_now_playing_msg;
use crate::providers::music::lavalink::Lavalink;
use bot_serenityutils::core::SHORT_TIMEOUT;
use bot_serenityutils::ephemeral_message::EphemeralMessage;

#[command]
#[only_in(guilds)]
#[description("Pauses playback")]
#[usage("")]
#[bucket("general")]
#[checks(DJ)]
async fn pause(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    log::debug!("Pausing playback for guild {}", guild.id);

    let queue = forward_error!(
        ctx,
        msg.channel_id,
        get_queue_for_guild(ctx, &guild.id).await
    );
    let mut queue_lock = queue.lock().await;

    if let Some(_) = queue_lock.current() {
        let is_paused = {
            let data = ctx.data.read().await;
            let player = data.get::<Lavalink>().unwrap();
            player.set_pause(guild.id.0, !queue_lock.paused()).await?;
            !queue_lock.paused()
        };
        queue_lock.set_paused(is_paused);
        if is_paused {
            log::debug!("Paused");
            EphemeralMessage::create(&ctx.http, msg.channel_id, SHORT_TIMEOUT, |m| {
                m.content("⏸️ Paused playback️")
            })
            .await?;
            if let (Some(menu), Some(song)) = (&queue_lock.now_playing_msg, queue_lock.current()) {
                update_now_playing_msg(&ctx.http, menu, &mut song.clone(), true).await?;
            }
        } else {
            log::debug!("Resumed");
            EphemeralMessage::create(&ctx.http, msg.channel_id, SHORT_TIMEOUT, |m| {
                m.content("▶ Resumed playback️")
            })
            .await?;
            if let (Some(menu), Some(song)) = (&queue_lock.now_playing_msg, queue_lock.current()) {
                update_now_playing_msg(&ctx.http, menu, &mut song.clone(), true).await?;
            }
        }
    } else {
        msg.channel_id.say(ctx, "Nothing to pause").await?;
    }
    handle_autodelete(ctx, msg).await?;

    Ok(())
}
