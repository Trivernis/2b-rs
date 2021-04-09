use crate::providers::settings::{get_setting, Setting};
use crate::utils::error::BotResult;
use serenity::model::channel::Message;
use serenity::prelude::*;

/// Deletes a message automatically if configured that way
pub async fn handle_autodelete(ctx: &Context, msg: &Message) -> BotResult<()> {
    if let Some(guild_id) = msg.guild_id {
        let autodelete = get_setting(ctx, guild_id, Setting::BotAutoDelete)
            .await?
            .unwrap_or(true);

        if autodelete {
            let _ = msg.delete(ctx).await;
        }
    }

    Ok(())
}
