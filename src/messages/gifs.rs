use crate::utils::error::BotResult;
use bot_database::models::Gif;
use bot_serenityutils::menu::{MenuBuilder, Page};
use serenity::builder::CreateMessage;
use serenity::client::Context;
use serenity::model::id::ChannelId;
use std::time::Duration;

/// Creates a new gifs embed
pub async fn create_gifs_menu(
    ctx: &Context,
    channel_id: ChannelId,
    gifs: Vec<Gif>,
) -> BotResult<()> {
    let total_pages = (gifs.len() as f32 / 10.0).ceil() as usize;
    let pages: Vec<Page> = gifs
        .chunks(10)
        .enumerate()
        .map(|(page, gifs)| create_gifs_page(page + 1, total_pages, gifs.to_vec()))
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
pub fn create_gifs_page(page: usize, total_pages: usize, gifs: Vec<Gif>) -> Page<'static> {
    let mut message = CreateMessage::default();
    let description_lines: Vec<String> = gifs
        .into_iter()
        .map(|g| {
            format!(
                "{} - {} - [Source]({})",
                g.category.unwrap_or("*N/A*".to_string()),
                g.name.unwrap_or("*N/A*".to_string()),
                g.url
            )
        })
        .collect();
    message.embed(|e| {
        e.title("Gifs")
            .description(description_lines.join("\n"))
            .footer(|f| f.text(format!("Page {} of {}", page, total_pages)))
    });

    Page::new_static(message)
}
