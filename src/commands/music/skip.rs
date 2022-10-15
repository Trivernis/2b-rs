use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{CommandError, CommandResult};
use serenity::model::channel::Message;

use crate::commands::common::handle_autodelete;
use crate::commands::music::{get_music_player_for_guild, DJ_CHECK};
use crate::messages::music::no_voicechannel::create_no_voicechannel_message;
use serenity_additions::core::SHORT_TIMEOUT;
use serenity_additions::ephemeral_message::EphemeralMessage;

#[command]
#[only_in(guilds)]
#[description("Skips to the next song")]
#[usage("")]
#[aliases("next")]
#[bucket("general")]
#[checks(DJ)]
async fn skip(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    tracing::debug!("Skipping song for guild {}", guild.id);

    let player = if let Some(player) = get_music_player_for_guild(ctx, guild.id).await {
        player
    } else {
        return create_no_voicechannel_message(&ctx.http, msg.channel_id)
            .await
            .map_err(CommandError::from);
    };
    {
        let mut player = player.lock().await;
        player.skip().await?;
    }

    EphemeralMessage::create(&ctx.http, msg.channel_id, SHORT_TIMEOUT, |m| {
        m.content("⏭ Skipped to the next song")
    })
    .await?;
    handle_autodelete(ctx, msg).await?;

    Ok(())
}
