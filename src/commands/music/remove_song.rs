use crate::commands::common::handle_autodelete;
use crate::commands::music::{get_queue_for_guild, DJ_CHECK};
use bot_serenityutils::core::SHORT_TIMEOUT;
use bot_serenityutils::ephemeral_message::EphemeralMessage;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;

#[command]
#[description("Removes a song from the queue")]
#[usage("<pos>")]
#[example("102")]
#[min_args(1)]
#[max_args(1)]
#[bucket("general")]
#[only_in(guilds)]
#[aliases("rms", "removesong", "remove-song")]
#[checks(DJ)]
async fn remove_song(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    log::debug!("Moving song for guild {}", guild.id);

    let pos = args.single::<usize>()?;

    {
        let queue = forward_error!(
            ctx,
            msg.channel_id,
            get_queue_for_guild(ctx, &guild.id).await
        );
        let mut queue_lock = queue.lock().await;
        queue_lock.remove(pos);
    }

    EphemeralMessage::create(&ctx.http, msg.channel_id, SHORT_TIMEOUT, |m| {
        m.content(format!("🗑️ Removed Song at `{}`", pos))
    })
    .await?;
    handle_autodelete(ctx, msg).await?;

    Ok(())
}
