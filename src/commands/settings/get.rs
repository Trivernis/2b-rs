use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;

use crate::database::get_database_from_context;
use crate::database::guild::GUILD_SETTINGS;

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
        let database_lock = database.lock().await;
        let setting = database_lock.get_guild_setting::<String>(&guild.id, &key);

        match setting {
            Ok(value) => {
                msg.channel_id
                    .say(ctx, format!("`{}` is set to to `{}`", key, value))
                    .await?;
            }
            Err(e) => {
                eprintln!("Failed to get setting: {:?}", e);
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
                let database_lock = database.lock().await;
                match database_lock.get_guild_setting::<String>(&guild.id, &key) {
                    Ok(value) => kv_pairs.push(format!("`{}` = `{}`", key, value)),
                    Err(e) => {
                        eprintln!("Failed to get setting: {:?}", e);
                        kv_pairs.push(format!("`{}` not set", key))
                    }
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
