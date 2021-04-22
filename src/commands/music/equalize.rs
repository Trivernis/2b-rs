use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandError, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::commands::common::handle_autodelete;
use crate::commands::music::{get_music_player_for_guild, DJ_CHECK};
use crate::messages::music::no_voicechannel::create_no_voicechannel_message;
use serenity_rich_interaction::core::{MEDIUM_TIMEOUT, SHORT_TIMEOUT};
use serenity_rich_interaction::ephemeral_message::EphemeralMessage;

#[command]
#[only_in(guilds)]
#[description("Loads an equalizer preset")]
#[usage("<preset>")]
#[num_args(1)]
#[example("bass")]
#[bucket("general")]
#[checks(DJ)]
async fn equalize(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    log::debug!("Changing equalizer for {}", guild.id);
    let preset = args.single::<String>().unwrap();

    let player = if let Some(player) = get_music_player_for_guild(ctx, guild.id).await {
        player
    } else {
        return create_no_voicechannel_message(&ctx.http, msg.channel_id)
            .await
            .map_err(CommandError::from);
    };

    let bands = match preset.to_lowercase().as_str() {
        "metal" => lavalink_rs::EQ_METAL,
        "boost" => lavalink_rs::EQ_BOOST,
        "base" => lavalink_rs::EQ_BASE,
        "piano" => lavalink_rs::EQ_PIANO,
        _ => {
            EphemeralMessage::create(&ctx.http, msg.channel_id, MEDIUM_TIMEOUT, |m| {
                m.content(format!(
                    "Unknown preset '{}'. Available are 'metal', 'boost', 'base' and 'piano'",
                    preset
                ))
            })
            .await?;
            handle_autodelete(ctx, msg).await?;
            return Ok(());
        }
    };
    {
        let mut player = player.lock().await;
        player.equalize_all(bands).await?;
    }
    EphemeralMessage::create(&ctx.http, msg.channel_id, SHORT_TIMEOUT, |m| {
        m.content(format!("🎛️ Changed equalizer to '{}'", preset))
    })
    .await?;

    handle_autodelete(ctx, msg).await?;

    Ok(())
}
