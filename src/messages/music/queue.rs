use crate::providers::music::queue::Song;
use crate::utils::error::BotResult;
use serenity::builder::CreateMessage;
use serenity::client::Context;
use serenity::model::id::ChannelId;
use serenity_additions::menu::{MenuBuilder, Page};
use std::time::Duration;

/// Creates a new queue menu
pub async fn create_queue_menu(
    ctx: &Context,
    channel_id: ChannelId,
    songs: Vec<(usize, Song)>,
) -> BotResult<()> {
    let page_count = (songs.len() as f32 / 10.0).ceil() as usize;
    let pages: Vec<Page<'static>> = songs
        .chunks(10)
        .enumerate()
        .map(|(i, entries)| create_songs_page(page_count, i + 1, entries.to_vec()))
        .collect();

    MenuBuilder::new_paginator()
        .add_pages(pages)
        .timeout(Duration::from_secs(120))
        .build(ctx, channel_id)
        .await?;

    Ok(())
}

/// Creates a new page with songs
fn create_songs_page(total_pages: usize, page: usize, songs: Vec<(usize, Song)>) -> Page<'static> {
    let mut message = CreateMessage::default();
    let description_entries: Vec<String> = songs
        .into_iter()
        .map(|(i, s)| format!("{:0>3}. {} - {}", i, s.author(), s.title()))
        .collect();
    message.embed(|e| {
        e.title("Queue")
            .description(format!("```md\n{}\n```", description_entries.join("\n")))
            .footer(|f| f.text(format!("Page {} of {}", page, total_pages)))
    });

    Page::new_static(message)
}
