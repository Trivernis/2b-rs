use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{CommandError, CommandResult};
use serenity::model::channel::Message;

use crate::commands::common::handle_autodelete;
use crate::commands::music::get_music_player_for_guild;
use crate::messages::music::no_voicechannel::create_no_voicechannel_message;
use crate::messages::music::now_playing::create_now_playing_msg;

#[command]
#[only_in(guilds)]
#[description("Displays the currently playing song")]
#[usage("")]
#[aliases("nowplaying", "np")]
#[bucket("general")]
async fn current(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();

    log::debug!("Displaying current song for queue in {}", guild.id);
    let player = if let Some(player) = get_music_player_for_guild(ctx, guild.id).await {
        player
    } else {
        return create_no_voicechannel_message(&ctx.http, msg.channel_id)
            .await
            .map_err(CommandError::from);
    };
    let current = {
        let mut player = player.lock().await;
        player.queue().current().clone()
    };

    if let Some(_) = current {
        let np_msg = create_now_playing_msg(ctx, player.clone(), msg.channel_id).await?;
        let mut player = player.lock().await;
        player.set_now_playing(np_msg).await;
    }
    handle_autodelete(ctx, msg).await?;

    Ok(())
}
