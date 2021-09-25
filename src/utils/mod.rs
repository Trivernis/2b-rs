use std::env;
use std::ops::Add;
use std::sync::Arc;
use std::time::SystemTime;

use lavalink_rs::LavalinkClient;
use serenity::client::Context;
use serenity::model::application::CurrentApplicationInfo;
use serenity::model::channel::Message;
use serenity::prelude::{RwLock, TypeMap};
use tokio::time::Instant;

use crate::providers::music::lavalink::{Lavalink, LavalinkHandler};
use crate::utils::context_data::get_database_from_context;
use crate::utils::error::BotResult;

pub(crate) mod context_data;
pub(crate) mod error;
pub(crate) mod logging;

#[macro_export]
macro_rules! forward_error {
    ($ctx:expr,$channel_id:expr,$result:expr) => {
        match $result {
            Err(e) => {
                $channel_id.say($ctx, format!("‼️ {}", e)).await?;
                return Ok(());
            }
            Ok(v) => v,
        }
    };
}

/// Returns the message the given message is a reply to or the message sent before that
pub async fn get_previous_message_or_reply(
    ctx: &Context,
    msg: &Message,
) -> BotResult<Option<Message>> {
    let referenced = if let Some(reference) = &msg.referenced_message {
        Some(*reference.clone())
    } else {
        let messages = msg
            .channel_id
            .messages(ctx, |ret| ret.before(&msg.id).limit(1))
            .await?;
        messages.first().cloned()
    };

    Ok(referenced)
}

/// Deletes all expired ephemeral messages that are stored in the database
pub async fn delete_messages_from_database(ctx: &Context) -> BotResult<()> {
    let database = get_database_from_context(ctx).await;
    let messages = database.get_ephemeral_messages().await?;

    for message in messages {
        if message.timeout <= SystemTime::now() {
            log::debug!("Deleting message {:?}", message);
            let _ = ctx
                .http
                .delete_message(message.channel_id as u64, message.message_id as u64)
                .await;
            database
                .delete_ephemeral_message(message.channel_id, message.message_id)
                .await?;
        } else {
            let http = Arc::clone(&ctx.http);
            let database = database.clone();
            log::debug!(
                "Creating future to delete ephemeral message {:?} later",
                message
            );

            tokio::spawn(async move {
                tokio::time::sleep_until(
                    Instant::now().add(message.timeout.duration_since(SystemTime::now()).unwrap()),
                )
                .await;
                log::debug!("Deleting message {:?}", message);
                let _ = http
                    .delete_message(message.channel_id as u64, message.message_id as u64)
                    .await;
                let _ = database
                    .delete_ephemeral_message(message.channel_id, message.message_id)
                    .await;
            });
        }
    }

    Ok(())
}

pub async fn initialize_lavalink(
    data: Arc<RwLock<TypeMap>>,
    current_application: CurrentApplicationInfo,
) -> BotResult<()> {
    let lava_client = LavalinkClient::builder(current_application.id.0)
        .set_host(env::var("LAVALINK_HOST").unwrap_or("172.0.0.1".to_string()))
        .set_password(env::var("LAVALINK_PASSWORD").expect("Missing lavalink password"))
        .set_port(
            env::var("LAVALINK_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .expect("Missing lavalink port"),
        )
        .build(LavalinkHandler {
            data: Arc::clone(&data),
        })
        .await?;
    {
        let mut data = data.write().await;
        data.insert::<Lavalink>(Arc::new(lava_client));
    }

    Ok(())
}
