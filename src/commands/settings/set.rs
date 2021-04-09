use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;

use crate::providers::settings::ALL_SETTINGS;
use crate::utils::context_data::get_database_from_context;

#[command]
#[only_in(guilds)]
#[description(
    "Set a guild setting. If no value is given the setting will be reset to the default value."
)]
#[usage("<setting> (<value>)")]
#[example("music.autoshuffle true")]
#[min_args(1)]
#[max_args(2)]
#[required_permissions("MANAGE_GUILD")]
async fn set(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let key = args.single::<String>().unwrap().to_lowercase();
    let all_settings: Vec<String> = ALL_SETTINGS.iter().map(|s| s.to_string()).collect();

    if !all_settings.contains(&key) {
        msg.channel_id
            .say(ctx, format!("Invalid setting `{}`", key))
            .await?;
        return Ok(());
    }
    let database = get_database_from_context(ctx).await;
    let guild = msg.guild(&ctx.cache).await.unwrap();

    if let Ok(value) = args.single::<String>() {
        database.set_guild_setting(guild.id.0, &key, value.clone())?;
        msg.channel_id
            .say(ctx, format!("Set `{}` to `{}`", key, value))
            .await?;
    } else {
        database.delete_guild_setting(guild.id.0, &key)?;
        msg.channel_id
            .say(ctx, format!("Setting `{}` reset to default", key))
            .await?;
    }

    Ok(())
}
