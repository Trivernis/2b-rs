use std::mem;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

use crate::commands::common::handle_autodelete;
use crate::commands::music::get_queue_for_guild;
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
    let queue = get_queue_for_guild(ctx, &guild.id).await?;

    let current = {
        let queue_lock = queue.lock().await;
        queue_lock.current().clone()
    };

    if let Some(current) = current {
        let metadata = current.metadata().clone();
        log::trace!("Metadata is {:?}", metadata);
        let np_msg = create_now_playing_msg(ctx, queue.clone(), msg.channel_id).await?;

        let mut queue_lock = queue.lock().await;
        if let Some(old_np) = mem::replace(&mut queue_lock.now_playing_msg, Some(np_msg)) {
            let old_np = old_np.read().await;
            if let Ok(message) = old_np.get_message(&ctx.http).await {
                let _ = message.delete(ctx).await;
            }
        }
    }
    handle_autodelete(ctx, msg).await?;

    Ok(())
}
