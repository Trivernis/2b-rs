use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

use crate::commands::common::handle_autodelete;
use crate::commands::music::get_queue_for_guild;
use crate::providers::music::lyrics::get_lyrics;

#[command]
#[only_in(guilds)]
#[description("Shows the lyrics for the currently playing song")]
#[usage("")]
#[bucket("general")]
async fn lyrics(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    log::debug!("Fetching lyrics for song playing in {}", guild.id);

    let queue = get_queue_for_guild(ctx, &guild.id).await?;
    let queue_lock = queue.lock().await;

    if let Some(current) = queue_lock.current() {
        log::debug!("Playing music. Fetching lyrics for currently playing song...");
        let metadata = current.metadata();
        let title = metadata.title.clone().unwrap();
        let author = metadata.artist.clone().unwrap();

        if let Some(lyrics) = get_lyrics(&*author, &*title).await? {
            log::trace!("Lyrics for '{}' are {}", title, lyrics);

            msg.channel_id
                .send_message(ctx, |m| {
                    m.embed(|e| {
                        e.title(format!("Lyrics for {} by {}", title, author))
                            .description(lyrics)
                            .footer(|f| f.text("Powered by lyricsovh"))
                    })
                })
                .await?;
        } else {
            log::debug!("No lyrics found");
            msg.channel_id.say(ctx, "No lyrics found").await?;
        }
    } else {
        msg.channel_id
            .say(ctx, "I'm not playing music right now")
            .await?;
    }
    handle_autodelete(ctx, msg).await?;

    Ok(())
}
