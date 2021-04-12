use std::collections::{HashMap, HashSet};

use bot_database::get_database;
use serenity::client::Context;
use serenity::framework::standard::macros::hook;
use serenity::framework::standard::{CommandResult, DispatchError};
use serenity::framework::StandardFramework;
use serenity::model::channel::Message;
use serenity::model::id::UserId;
use serenity::Client;
use songbird::SerenityInit;

use crate::commands::*;
use crate::handler::Handler;
use crate::utils::context_data::{DatabaseContainer, Store, StoreData};
use crate::utils::error::{BotError, BotResult};
use bot_serenityutils::menu::EventDrivenMessageContainer;
use serenity::framework::standard::buckets::LimitedFor;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn get_client() -> BotResult<Client> {
    let token = dotenv::var("BOT_TOKEN").map_err(|_| BotError::MissingToken)?;
    let database = get_database()?;

    let client = Client::builder(token)
        .event_handler(Handler)
        .framework(get_framework().await)
        .register_songbird()
        .await?;
    {
        let mut data = client.data.write().await;
        data.insert::<Store>(StoreData::new());
        data.insert::<DatabaseContainer>(database);
        data.insert::<EventDrivenMessageContainer>(Arc::new(Mutex::new(HashMap::new())));
    }

    Ok(client)
}

pub async fn get_framework() -> StandardFramework {
    let mut owners = HashSet::new();
    if let Some(owner) = dotenv::var("BOT_OWNER").ok().and_then(|o| o.parse().ok()) {
        owners.insert(UserId(owner));
    }
    StandardFramework::default()
        .configure(|c| {
            c.prefix(
                dotenv::var("BOT_PREFIX")
                    .unwrap_or("~!".to_string())
                    .as_str(),
            )
            .allow_dm(true)
            .ignore_bots(true)
            .owners(owners)
        })
        .group(&MINECRAFT_GROUP)
        .group(&MISC_GROUP)
        .group(&MUSIC_GROUP)
        .group(&SETTINGS_GROUP)
        .after(after_hook)
        .before(before_hook)
        .on_dispatch_error(dispatch_error)
        .help(&HELP)
        .bucket("music_api", |b| {
            b.delay(1)
                .time_span(60)
                .limit(30)
                .limit_for(LimitedFor::User)
        })
        .await
        .bucket("sauce_api", |b| {
            b.delay(1)
                .time_span(60)
                .limit(10)
                .limit_for(LimitedFor::User)
        })
        .await
        .bucket("general", |b| b.time_span(10).limit(5))
        .await
}

#[hook]
async fn after_hook(ctx: &Context, msg: &Message, cmd_name: &str, error: CommandResult) {
    //  Print out an error if it happened
    if let Err(why) = error {
        let _ = msg
            .channel_id
            .send_message(ctx, |m| {
                m.embed(|e| e.title("Error occurred").description(format!("{}", why)))
            })
            .await;
        log::warn!("Error in {}: {:?}", cmd_name, why);
    }
}

#[hook]
async fn before_hook(ctx: &Context, msg: &Message, _: &str) -> bool {
    log::trace!("Got command message {}", msg.content);
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
