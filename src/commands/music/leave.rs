use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

use crate::commands::common::handle_autodelete;
use crate::commands::music::DJ_CHECK;
use crate::utils::context_data::MusicPlayers;
use bot_serenityutils::core::SHORT_TIMEOUT;
use bot_serenityutils::ephemeral_message::EphemeralMessage;

#[command]
#[only_in(guilds)]
#[description("Leaves a voice channel")]
#[usage("")]
#[aliases("stop")]
#[bucket("general")]
#[checks(DJ)]
async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    log::debug!("Leave request received for guild {}", guild.id);

    let manager = songbird::get(ctx).await.unwrap();
    if let Some(handler) = manager.get(guild.id) {
        let mut handler_lock = handler.lock().await;
        let _ = handler_lock.leave().await;
    }

    let mut data = ctx.data.write().await;
    let players = data.get_mut::<MusicPlayers>().unwrap();

    match players.remove(&guild.id.0) {
        None => {
            EphemeralMessage::create(&ctx.http, msg.channel_id, SHORT_TIMEOUT, |m| {
                m.content("‼️ I'm not in a Voice Channel")
            })
            .await?;
        }
        Some(player) => {
            let mut player = player.lock().await;
            player.stop().await?;
            player.delete_now_playing().await?;
        }
    }
    manager.remove(guild.id).await?;

    handle_autodelete(ctx, msg).await?;

    Ok(())
}
