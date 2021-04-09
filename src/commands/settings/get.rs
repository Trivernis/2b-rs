use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;

use crate::providers::constants::GUILD_SETTINGS;
use crate::utils::context_data::get_database_from_context;

#[command]
#[only_in(guilds)]
#[description("Get a guild setting")]
#[usage("(<setting>)")]
#[example("music.autoshuffle")]
#[min_args(0)]
#[max_args(1)]
#[required_permissions("MANAGE_GUILD")]
async fn get(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let database = get_database_from_context(ctx).await;
    let guild = msg.guild(&ctx.cache).await.unwrap();
    log::debug!("Displaying guild setting for guild {}", guild.id);

    if let Some(key) = args.single::<String>().ok() {
        log::debug!("Displaying guild setting of '{}'", key);
        let setting = database.get_guild_setting::<String>(guild.id.0, &key)?;

        match setting {
            Some(value) => {
                msg.channel_id
                    .say(ctx, format!("`{}` is set to to `{}`", key, value))
                    .await?;
            }
            None => {
                msg.channel_id
                    .say(ctx, format!("`{}` is not set", key))
                    .await?;
            }
        }
    } else {
        log::debug!("Displaying all guild settings");
        for key in GUILD_SETTINGS {
            let mut kv_pairs = Vec::new();
            {
                match database.get_guild_setting::<String>(guild.id.0, &key)? {
                    Some(value) => kv_pairs.push(format!("`{}` = `{}`", key, value)),
                    None => kv_pairs.push(format!("`{}` not set", key)),
                }
            }
            msg.channel_id
                .send_message(ctx, |m| {
                    m.embed(|e| e.title("Guild Settings").description(kv_pairs.join("\n")))
                })
                .await?;
        }
    }

    Ok(())
}
