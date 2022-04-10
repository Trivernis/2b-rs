use crate::commands::common::handle_autodelete;
use crate::commands::music::{get_music_player_for_guild, DJ_CHECK};
use crate::messages::music::no_voicechannel::create_no_voicechannel_message;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandError, CommandResult};
use serenity::model::channel::Message;
use serenity_rich_interaction::core::SHORT_TIMEOUT;
use serenity_rich_interaction::ephemeral_message::EphemeralMessage;

#[command]
#[description("Moves a song in the queue from one position to a new one")]
#[usage("<old-pos> <new-pos>")]
#[example("102 2")]
#[num_args(2)]
#[bucket("general")]
#[only_in(guilds)]
#[aliases("mvs", "movesong", "move-song")]
#[checks(DJ)]
async fn move_song(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    tracing::debug!("Moving song for guild {}", guild.id);

    let pos1 = args.single::<usize>()?;
    let pos2 = args.single::<usize>()?;
    let player = if let Some(player) = get_music_player_for_guild(ctx, guild.id).await {
        player
    } else {
        return create_no_voicechannel_message(&ctx.http, msg.channel_id)
            .await
            .map_err(CommandError::from);
    };

    {
        let mut player = player.lock().await;
        player.queue().move_position(pos1, pos2);
    }
    EphemeralMessage::create(&ctx.http, msg.channel_id, SHORT_TIMEOUT, |m| {
        m.content(format!(
            "↕ Moved Song `{}` to new position `{}`️",
            pos1, pos2
        ))
    })
    .await?;
    handle_autodelete(ctx, msg).await?;

    Ok(())
}
