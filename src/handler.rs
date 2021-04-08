use serenity::async_trait;
use serenity::client::Context;
use serenity::model::event::ResumedEvent;
use serenity::model::gateway::{Activity, Ready};
use serenity::prelude::*;

pub(crate) struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        log::info!("Connected as {}", ready.user.name);
        let prefix = dotenv::var("BOT_PREFIX").unwrap_or("~!".to_string());
        ctx.set_activity(Activity::listening(format!("{}help", prefix).as_str()))
            .await;
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        log::info!("Reconnected to gateway")
    }
}
