use crate::utils::error::BotResult;
use bot_database::models::Media;
use serenity::builder::CreateMessage;
use serenity::client::Context;
use serenity::model::id::ChannelId;
use serenity_rich_interaction::menu::{MenuBuilder, Page};
use std::time::Duration;

/// Creates a new gifs embed
pub async fn create_media_menu(
    ctx: &Context,
    channel_id: ChannelId,
    media: Vec<Media>,
) -> BotResult<()> {
    let total_pages = (media.len() as f32 / 10.0).ceil() as usize;
    let pages: Vec<Page> = media
        .chunks(10)
        .enumerate()
        .map(|(page, media)| create_media_page(page + 1, total_pages, media.to_vec()))
        .collect();

    MenuBuilder::new_paginator()
        .timeout(Duration::from_secs(120))
        .add_pages(pages)
        .show_help()
        .build(ctx, channel_id)
        .await?;

    Ok(())
}

/// Creates a new gif page
pub fn create_media_page(page: usize, total_pages: usize, media: Vec<Media>) -> Page<'static> {
    let mut message = CreateMessage::default();
    let description_lines: Vec<String> = media
        .into_iter()
        .map(|m| {
            format!(
                "{} - {} - [Source]({})",
                m.category.unwrap_or("*N/A*".to_string()),
                m.name.unwrap_or("*N/A*".to_string()),
                m.url
            )
        })
        .collect();
    message.embed(|e| {
        e.title("Media")
            .description(description_lines.join("\n"))
            .footer(|f| f.text(format!("Page {} of {}", page, total_pages)))
    });

    Page::new_static(message)
}
