use serenity::async_trait;
use serenity::client::{Context, EventHandler};
use serenity::framework::standard::macros::hook;
use serenity::framework::standard::{CommandResult, DispatchError};
use serenity::framework::StandardFramework;
use serenity::model::channel::Message;
use serenity::Client;
use songbird::SerenityInit;

use crate::commands::*;
use crate::database::get_database;
use crate::utils::error::{BotError, BotResult};
use crate::utils::store::{Store, StoreData};

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

pub async fn get_client() -> BotResult<Client> {
    let token = dotenv::var("BOT_TOKEN").map_err(|_| BotError::MissingToken)?;
    let database = get_database()?;

    let client = Client::builder(token)
        .framework(get_framework())
        .register_songbird()
        .await?;
    {
        let mut data = client.data.write().await;
        data.insert::<Store>(StoreData::new(database))
    }

    Ok(client)
}

pub fn get_framework() -> StandardFramework {
    StandardFramework::default()
        .configure(|c| {
            c.prefix(
                dotenv::var("BOT_PREFIX")
                    .unwrap_or("~!".to_string())
                    .as_str(),
            )
            .allow_dm(true)
            .ignore_bots(true)
        })
        .group(&MINECRAFT_GROUP)
        .group(&MISC_GROUP)
        .group(&MUSIC_GROUP)
        .group(&SETTINGS_GROUP)
        .after(after_hook)
        .before(before_hook)
        .on_dispatch_error(dispatch_error)
        .help(&HELP)
}

#[hook]
async fn after_hook(ctx: &Context, msg: &Message, cmd_name: &str, error: CommandResult) {
    //  Print out an error if it happened
    if let Err(why) = error {
        let _ = msg
            .channel_id
            .send_message(ctx, |m| {
                m.embed(|e| {
                    e.title("Error occurred")
                        .description(format!("```\n{}\n```", why))
                })
            })
            .await;
        println!("Error in {}: {:?}", cmd_name, why);
    }
}

#[hook]
async fn before_hook(ctx: &Context, msg: &Message, _: &str) -> bool {
    let _ = msg.channel_id.broadcast_typing(ctx).await;
    true
}

#[hook]
async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError) {
    match error {
        DispatchError::Ratelimited(info) => {
            if info.is_first_try {
                let _ = msg
                    .channel_id
                    .say(
                        &ctx.http,
                        &format!("Try this again in {} seconds.", info.as_secs()),
                    )
                    .await;
            }
        }
        DispatchError::OnlyForDM => {
            let _ = msg
                .channel_id
                .say(&ctx.http, "This command only works via DM")
                .await;
        }
        DispatchError::OnlyForGuilds => {
            let _ = msg
                .channel_id
                .say(&ctx.http, "This command only works on servers")
                .await;
        }
        DispatchError::NotEnoughArguments { min, given } => {
            let _ = msg
                .channel_id
                .say(
                    &ctx.http,
                    format!("Expected {} arguments but only got {}", min, given),
                )
                .await;
        }
        DispatchError::TooManyArguments { max, given } => {
            let _ = msg
                .channel_id
                .say(
                    &ctx.http,
                    format!("Expected {} arguments but actually got {}", max, given),
                )
                .await;
        }
        _ => {}
    }
}
