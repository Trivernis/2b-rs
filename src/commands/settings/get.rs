use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;

use crate::providers::settings::ALL_SETTINGS;
use crate::utils::context_data::get_database_from_context;

#[command]
#[only_in(guilds)]
#[description("Get a guild setting")]
#[usage("[<setting>]")]
#[example("music.autoshuffle")]
#[min_args(0)]
#[max_args(1)]
#[required_permissions("MANAGE_GUILD")]
#[bucket("general")]
async fn get(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let database = get_database_from_context(ctx).await;
    let guild = msg.guild(&ctx.cache).unwrap();
    tracing::debug!("Displaying guild setting for guild {}", guild.id);

    if let Some(key) = args.single::<String>().ok() {
        tracing::debug!("Displaying guild setting of '{}'", key);
        let setting = database
            .get_guild_setting::<String, _>(guild.id.0, key.clone())
            .await?;

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
        tracing::debug!("Displaying all guild settings");
        let mut kv_pairs = Vec::new();

        for key in ALL_SETTINGS {
            let key = key.to_string();

            {
                match database
                    .get_guild_setting::<String, _>(guild.id.0, key.clone())
                    .await?
                {
                    Some(value) => kv_pairs.push(format!("`{}` = `{}`", key, value)),
                    None => kv_pairs.push(format!("`{}` not set", key)),
                }
            }
        }
        msg.channel_id
            .send_message(ctx, |m| {
                m.embed(|e| e.title("Guild Settings").description(kv_pairs.join("\n")))
            })
            .await?;
    }

    Ok(())
}
