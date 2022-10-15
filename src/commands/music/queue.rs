use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandError, CommandResult};
use serenity::model::channel::Message;

use crate::commands::common::handle_autodelete;
use crate::commands::music::get_music_player_for_guild;
use crate::messages::music::no_voicechannel::create_no_voicechannel_message;
use crate::messages::music::queue::create_queue_menu;
use crate::providers::music::queue::Song;

#[command]
#[only_in(guilds)]
#[description("Shows the song queue")]
#[usage("(<query...>)")]
#[aliases("q")]
#[bucket("general")]
async fn queue(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    tracing::trace!("Displaying queue for guild {}", guild.id);

    let query = args
        .iter::<String>()
        .map(|s| s.unwrap().to_lowercase())
        .collect::<Vec<String>>();

    let player = if let Some(player) = get_music_player_for_guild(ctx, guild.id).await {
        player
    } else {
        return create_no_voicechannel_message(&ctx.http, msg.channel_id)
            .await
            .map_err(CommandError::from);
    };
    let mut player = player.lock().await;
    let songs: Vec<(usize, Song)> = player
        .queue()
        .entries()
        .into_iter()
        .enumerate()
        .filter(|(i, s)| {
            if query.is_empty() {
                return true;
            }
            for kw in &query {
                if s.title().to_lowercase().contains(kw)
                    || s.author().to_lowercase().contains(kw)
                    || &i.to_string() == kw
                {
                    return true;
                }
            }
            false
        })
        .map(|(i, s)| (i, s.clone()))
        .collect();
    tracing::trace!("Songs are {:?}", songs);

    if songs.len() == 0 {
        msg.channel_id
            .send_message(ctx, |m| {
                m.embed(|e| e.title("Queue").description("*The queue is empty*"))
            })
            .await?;

        return Ok(());
    }
    create_queue_menu(ctx, msg.channel_id, songs).await?;

    handle_autodelete(ctx, msg).await?;

    Ok(())
}
