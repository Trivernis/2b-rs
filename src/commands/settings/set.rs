use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;

use crate::database::get_database_from_context;

#[command]
#[only_in(guilds)]
#[description("Set a guild setting")]
#[usage("set <setting> <value>")]
#[example("set music.autoshuffle true")]
#[min_args(2)]
#[max_args(2)]
#[required_permissions("MANAGE_GUILD")]
async fn set(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let key = args.single::<String>().unwrap();
    let value = args.single::<String>().unwrap();
    let database = get_database_from_context(ctx).await;
    let database_lock = database.lock().await;
    let guild = msg.guild(&ctx.cache).await.unwrap();
    database_lock.set_guild_setting(&guild.id, &key, value.clone())?;
    msg.channel_id
        .say(ctx, format!("Set `{}` to `{}`", key, value))
        .await?;

    Ok(())
}
