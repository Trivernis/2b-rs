use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{CommandError, CommandResult};
use serenity::model::channel::Message;

use crate::commands::common::handle_autodelete;
use crate::commands::music::get_music_player_for_guild;
use crate::messages::music::no_voicechannel::create_no_voicechannel_message;

#[command]
#[only_in(guilds)]
#[description("Shows the lyrics for the currently playing song")]
#[usage("")]
#[bucket("general")]
async fn lyrics(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    tracing::debug!("Fetching lyrics for song playing in {}", guild.id);

    let player = if let Some(player) = get_music_player_for_guild(ctx, guild.id).await {
        player
    } else {
        return create_no_voicechannel_message(&ctx.http, msg.channel_id)
            .await
            .map_err(CommandError::from);
    };

    let (lyrics, current) = {
        let mut player = player.lock().await;
        let current = player.queue().current().clone();
        (player.lyrics().await?, current)
    };

    if let Some(lyrics) = lyrics {
        let current = current.unwrap();
        msg.channel_id
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.title(format!(
                        "Lyrics for {} by {}",
                        current.title(),
                        current.author()
                    ))
                    .description(lyrics)
                    .footer(|f| f.text("Powered by lyricsovh"))
                })
            })
            .await?;
    } else {
        tracing::debug!("No lyrics found");
        msg.channel_id.say(ctx, "No lyrics found").await?;
    }

    handle_autodelete(ctx, msg).await?;

    Ok(())
}
