use std::mem;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

use crate::commands::common::handle_autodelete;
use crate::commands::music::get_queue_for_guild;
use crate::messages::music::NowPlayingMessage;

#[command]
#[only_in(guilds)]
#[description("Displays the currently playing song")]
#[usage("")]
#[aliases("nowplaying", "np")]
async fn current(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();

    log::debug!("Displaying current song for queue in {}", guild.id);
    let queue = get_queue_for_guild(ctx, &guild.id).await?;
    let mut queue_lock = queue.lock().await;

    if let Some(current) = queue_lock.current() {
        let metadata = current.metadata().clone();
        log::trace!("Metadata is {:?}", metadata);
        let np_msg =
            NowPlayingMessage::create(ctx.http.clone(), &msg.channel_id, &metadata).await?;

        if let Some(old_np) = mem::replace(&mut queue_lock.now_playing_msg, Some(np_msg)) {
            let _ = old_np.inner().delete().await;
        }
    }
    handle_autodelete(ctx, msg).await?;

    Ok(())
}
