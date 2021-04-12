use crate::commands::common::handle_autodelete;
use crate::commands::music::{get_queue_for_guild, is_dj};
use bot_serenityutils::core::SHORT_TIMEOUT;
use bot_serenityutils::ephemeral_message::EphemeralMessage;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;

#[command]
#[description("Moves a song in the queue from one position to a new one")]
#[usage("<old-pos> <new-pos>")]
#[example("102 2")]
#[min_args(2)]
#[max_args(2)]
#[bucket("general")]
#[only_in(guilds)]
#[aliases("mvs", "movesong", "move-song")]
async fn move_song(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    log::debug!("Moving song for guild {}", guild.id);

    let pos1 = args.single::<usize>()?;
    let pos2 = args.single::<usize>()?;

    if !is_dj(ctx, guild.id, &msg.author).await? {
        msg.channel_id.say(ctx, "Requires DJ permissions").await?;
        return Ok(());
    }
    {
        let queue = get_queue_for_guild(ctx, &guild.id).await?;
        let mut queue_lock = queue.lock().await;
        queue_lock.move_position(pos1, pos2);
    }
    EphemeralMessage::create(&ctx.http, msg.channel_id, SHORT_TIMEOUT, |m| {
        m.content(format!(
            "↕ Moved Song `{}` to new position `{}`️",
            pos1, pos2
        ))
    })
    .await?;
    handle_autodelete(ctx, msg).await?;

    Ok(())
}
