use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

use crate::commands::common::handle_autodelete;
use crate::commands::music::{get_voice_manager, is_dj};
use crate::utils::context_data::Store;
use bot_serenityutils::core::SHORT_TIMEOUT;
use bot_serenityutils::ephemeral_message::EphemeralMessage;

#[command]
#[only_in(guilds)]
#[description("Leaves a voice channel")]
#[usage("")]
#[aliases("stop")]
#[bucket("general")]
async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    log::debug!("Leave request received for guild {}", guild.id);
    if !is_dj(ctx, guild.id, &msg.author).await? {
        msg.channel_id.say(ctx, "Requires DJ permissions").await?;
        return Ok(());
    }

    let manager = get_voice_manager(ctx).await;
    let queue = {
        let mut data = ctx.data.write().await;
        let store = data.get_mut::<Store>().unwrap();
        store
            .music_queues
            .remove(&guild.id)
            .expect("No queue for guild.")
    };
    let queue_lock = queue.lock().await;
    let handler = manager.get(guild.id);

    if let Some(handler) = handler {
        let mut handler_lock = handler.lock().await;
        handler_lock.remove_all_global_events();
    }

    if manager.get(guild.id).is_some() {
        if let Some(current) = queue_lock.current() {
            current.stop()?;
        }
        manager.remove(guild.id).await?;
        EphemeralMessage::create(&ctx.http, msg.channel_id, SHORT_TIMEOUT, |m| {
            m.content("👋 Left the Voice Channel")
        })
        .await?;
        log::debug!("Left the voice channel");
    } else {
        EphemeralMessage::create(&ctx.http, msg.channel_id, SHORT_TIMEOUT, |m| {
            m.content("‼️ I'm not in a Voice Channel")
        })
        .await?;
        log::debug!("Not in a voice channel");
    }
    handle_autodelete(ctx, msg).await?;

    Ok(())
}
