use crate::commands::music::{get_queue_for_guild, is_dj};
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
async fn remove_song(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    log::debug!("Moving song for guild {}", guild.id);

    let pos = args.single::<usize>()?;

    if !is_dj(ctx, guild.id, &msg.author).await? {
        msg.channel_id.say(ctx, "Requires DJ permissions").await?;
        return Ok(());
    }
    {
        let queue = get_queue_for_guild(ctx, &guild.id).await?;
        let mut queue_lock = queue.lock().await;
        queue_lock.remove(pos);
    }

    msg.channel_id
        .say(ctx, format!("Removed Song at `{}`", pos))
        .await?;

    Ok(())
}
