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
use crate::providers::music::lavalink::{Lavalink, LavalinkHandler};
use crate::utils::context_data::{
    get_database_from_context, DatabaseContainer, MusicPlayers, Store, StoreData,
};
use crate::utils::error::{BotError, BotResult};
use bot_serenityutils::menu::EventDrivenMessageContainer;
use lavalink_rs::LavalinkClient;
use serenity::framework::standard::buckets::LimitedFor;
use std::env;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::Mutex;

pub async fn get_client() -> BotResult<Client> {
    let token = env::var("BOT_TOKEN").map_err(|_| BotError::MissingToken)?;
    let database = get_database()?;
    let client = Client::builder(token)
        .event_handler(Handler)
        .framework(get_framework().await)
        .register_songbird()
        .type_map_insert::<Store>(StoreData::new())
        .type_map_insert::<DatabaseContainer>(database)
        .type_map_insert::<MusicPlayers>(HashMap::new())
        .type_map_insert::<EventDrivenMessageContainer>(Arc::new(Mutex::new(HashMap::new())))
        .await?;
    let data = client.data.clone();

    let current_application = client
        .cache_and_http
        .http
        .get_current_application_info()
        .await?;

    let lava_client = LavalinkClient::builder(current_application.id.0)
        .set_host(env::var("LAVALINK_HOST").unwrap_or("172.0.0.1".to_string()))
        .set_password(env::var("LAVALINK_PASSWORD").expect("Missing lavalink password"))
        .set_port(
            env::var("LAVALINK_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .expect("Missing lavalink port"),
        )
        .build(LavalinkHandler { data })
        .await?;
    {
        let mut data = client.data.write().await;
        data.insert::<Lavalink>(Arc::new(lava_client));
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
        .group(&WEEB_GROUP)
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
    let mut error_msg = None;
    if let Err(why) = error {
        error_msg = Some(why.to_string());
        let _ = msg
            .channel_id
            .send_message(ctx, |m| {
                m.embed(|e| e.title("Error occurred").description(format!("{}", why)))
            })
            .await;
        log::warn!("Error in {}: {:?}", cmd_name, why);
    }
    let database = get_database_from_context(ctx).await;
    let _ = database
        .add_statistic(
            crate::VERSION,
            cmd_name,
            SystemTime::now(),
            error_msg.is_none(),
            error_msg,
        )
        .await;
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
