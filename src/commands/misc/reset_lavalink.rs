use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

use crate::commands::common::handle_autodelete;
use crate::providers::music::lavalink::Lavalink;
use crate::utils::initialize_lavalink;
use serenity_rich_interaction::core::SHORT_TIMEOUT;
use serenity_rich_interaction::ephemeral_message::EphemeralMessage;
use std::mem;
use std::sync::Arc;

#[command]
#[description("Resets the lavalink connection")]
#[aliases("reconnect_lavalink", "reset-lavalink", "reconnect-lavalink")]
#[num_args(0)]
#[owners_only]
async fn reset_lavalink(ctx: &Context, msg: &Message) -> CommandResult {
    let app_info = ctx.http.get_current_application_info().await?;
    destroy_lavalink(ctx).await;

    initialize_lavalink(Arc::clone(&ctx.data), app_info).await?;

    EphemeralMessage::create(&ctx.http, msg.channel_id, SHORT_TIMEOUT, |m| {
        m.content("Reconnected to lavalink")
    })
    .await?;
    handle_autodelete(ctx, msg).await?;

    Ok(())
}

async fn destroy_lavalink(ctx: &Context) {
    let mut data = ctx.data.write().await;
    {
        let lava_client = data.remove::<Lavalink>().unwrap();
        mem::drop(lava_client);
    }
}
