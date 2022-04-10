use sauce_api::Sauce;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

use bot_coreutils::url;

use crate::messages::sauce::show_sauce_menu;
use crate::utils::context_data::Store;
use crate::utils::get_previous_message_or_reply;

#[command]
#[description("Searches for the source of a previously posted image or an image replied to.")]
#[usage("")]
#[aliases("source")]
#[bucket("sauce_api")]
async fn sauce(ctx: &Context, msg: &Message) -> CommandResult {
    tracing::debug!("Got sauce command");
    let source_msg = get_previous_message_or_reply(ctx, msg).await?;

    if source_msg.is_none() {
        tracing::debug!("No source message provided");
        msg.channel_id.say(ctx, "No source message found.").await?;
        return Ok(());
    }
    let source_msg = source_msg.unwrap();
    tracing::trace!("Source message is {:?}", source_msg);
    tracing::debug!("Getting attachments...");
    let mut attachment_urls: Vec<String> = source_msg
        .attachments
        .into_iter()
        .map(|a| a.url)
        .filter(|url| url::is_image(url) || url::is_video(url))
        .collect();

    tracing::debug!("Getting embedded images...");
    let mut embed_images = source_msg
        .embeds
        .into_iter()
        .filter_map(|e| e.thumbnail.map(|t| t.url).or(e.image.map(|i| i.url)))
        .filter(|url| url::is_image(url) || url::is_video(url))
        .collect::<Vec<String>>();

    attachment_urls.append(&mut embed_images);
    tracing::trace!("Image urls {:?}", attachment_urls);

    if attachment_urls.is_empty() {
        tracing::debug!("No images in source image");
        msg.channel_id
            .say(ctx, "I could not find any images in the message.")
            .await?;
        return Ok(());
    }

    tracing::debug!(
        "Checking SauceNao for {} attachments",
        attachment_urls.len()
    );
    let data = ctx.data.read().await;
    let store_data = data.get::<Store>().unwrap();
    let sources = store_data
        .sauce_nao
        .check_sauces(&attachment_urls[..])
        .await?;
    tracing::trace!("Sources are {:?}", sources);

    tracing::debug!("Creating menu...");

    show_sauce_menu(ctx, msg, sources).await?;
    tracing::debug!("Menu created");

    Ok(())
}
