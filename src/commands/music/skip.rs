use serenity::client::Context;
use serenity::framework::standard::{CommandError, CommandResult};
use serenity::framework::standard::macros::command;
use serenity::model::channel::Message;

#[command]
#[only_in(guilds)]
#[description("Skips to the next song")]
#[usage("skip")]
#[aliases("next")]
async fn skip(ctx: &Context, msg: &Message) -> CommandResult {
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
        current.stop()?;
    }
    handler.queue().skip()?;
    msg.channel_id.say(ctx, "Skipped to the next song").await?;

    Ok(())
}
