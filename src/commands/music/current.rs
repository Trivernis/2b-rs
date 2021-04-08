use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

use crate::commands::music::get_queue_for_guild;

#[command]
#[only_in(guilds)]
#[description("Displays the currently playing song")]
#[usage("current")]
#[aliases("nowplaying", "np")]
async fn current(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();

    log::debug!("Displaying current song for queue in {}", guild.id);
    let queue = get_queue_for_guild(ctx, &guild.id).await?;
    let queue_lock = queue.lock().await;

    if let Some(current) = queue_lock.current() {
        let metadata = current.metadata().clone();
        log::trace!("Metadata is {:?}", metadata);
        msg.channel_id
            .send_message(ctx, |m| {
                m.embed(|mut e| {
                    e = e.description(format!(
                        "Now Playing [{}]({}) by {}",
                        metadata.title.unwrap(),
                        metadata.source_url.unwrap(),
                        metadata.artist.unwrap()
                    ));

                    if let Some(thumb) = metadata.thumbnail {
                        e = e.thumbnail(thumb);
                    }

                    e
                })
            })
            .await?;
    }

    Ok(())
}
