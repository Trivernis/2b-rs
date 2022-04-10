use serenity::framework::standard::macros::command;
use serenity::framework::standard::{CommandError, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

use crate::commands::common::handle_autodelete;
use crate::commands::music::{get_music_player_for_guild, DJ_CHECK};
use crate::messages::music::equalizer::create_equalizer_message;
use crate::messages::music::no_voicechannel::create_no_voicechannel_message;

#[command]
#[only_in(guilds)]
#[description("Displays the equalizer for the music player")]
#[usage("")]
#[bucket("general")]
#[checks(DJ)]
async fn equalizer(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    tracing::debug!("Displaying equalizer for guild {}", guild.id);

    let player = if let Some(player) = get_music_player_for_guild(ctx, guild.id).await {
        player
    } else {
        return create_no_voicechannel_message(&ctx.http, msg.channel_id)
            .await
            .map_err(CommandError::from);
    };

    create_equalizer_message(&ctx, msg.channel_id, player).await?;
    handle_autodelete(ctx, msg).await?;

    Ok(())
}
