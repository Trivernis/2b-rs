use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

use crate::commands::common::handle_autodelete;
use crate::commands::music::DJ_CHECK;
use crate::providers::music::lavalink::Lavalink;
use bot_serenityutils::core::SHORT_TIMEOUT;
use bot_serenityutils::ephemeral_message::EphemeralMessage;

#[command]
#[only_in(guilds)]
#[description("Skips to the next song")]
#[usage("")]
#[aliases("next")]
#[bucket("general")]
#[checks(DJ)]
async fn skip(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    log::debug!("Skipping song for guild {}", guild.id);

    {
        let data = ctx.data.read().await;
        let player = data.get::<Lavalink>().unwrap();
        player.stop(guild.id.0).await?;
    }

    EphemeralMessage::create(&ctx.http, msg.channel_id, SHORT_TIMEOUT, |m| {
        m.content("⏭ Skipped to the next song")
    })
    .await?;
    handle_autodelete(ctx, msg).await?;

    Ok(())
}
