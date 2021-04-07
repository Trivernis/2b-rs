use serenity::client::Context;
use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::command;
use serenity::model::channel::Message;

#[command]
#[only_in(guilds)]
#[description("Leaves a voice channel")]
#[usage("leave")]
#[aliases("stop")]
async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if manager.get(guild_id).is_some() {
        manager.remove(guild_id).await?;
        msg.channel_id.say(ctx, "Left the voice channel").await?;
    } else {
        msg.channel_id.say(ctx, "Not in a voice channel").await?;
    }

    Ok(())
}
