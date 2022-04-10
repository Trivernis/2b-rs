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
#[description("Removes a song from the queue")]
#[usage("<pos>")]
#[example("102")]
#[num_args(1)]
#[bucket("general")]
#[only_in(guilds)]
#[aliases("rms", "removesong", "remove-song")]
#[checks(DJ)]
async fn remove_song(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    tracing::debug!("Moving song for guild {}", guild.id);

    let pos = args.single::<usize>()?;
    let player = if let Some(player) = get_music_player_for_guild(ctx, guild.id).await {
        player
    } else {
        return create_no_voicechannel_message(&ctx.http, msg.channel_id)
            .await
            .map_err(CommandError::from);
    };

    {
        let mut player = player.lock().await;
        player.queue().remove(pos);
    }

    EphemeralMessage::create(&ctx.http, msg.channel_id, SHORT_TIMEOUT, |m| {
        m.content(format!("🗑️ Removed Song at `{}`", pos))
    })
    .await?;
    handle_autodelete(ctx, msg).await?;

    Ok(())
}
