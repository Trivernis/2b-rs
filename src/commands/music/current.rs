use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{CommandError, CommandResult};
use serenity::model::channel::Message;

#[command]
#[only_in(guilds)]
#[description("Displays the currently playing song")]
#[usage("current")]
#[aliases("nowplaying", "np")]
async fn current(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let handler_lock = manager
        .get(guild.id)
        .ok_or(CommandError::from("Not in a voice channel"))?;
    let handler = handler_lock.lock().await;

    if let Some(current) = handler.queue().current() {
        let metadata = current.metadata().clone();
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
