use crate::utils::error::BotResult;
use animethemes_rs::models::{Anime, ThemeEntry, ThemeType};
use serenity::builder::CreateMessage;
use serenity::client::Context;
use serenity::model::id::{ChannelId, UserId};
use serenity_additions::core::EXTRA_LONG_TIMEOUT;
use serenity_additions::menu::{MenuBuilder, Page};

/// Creates a new Anime Theme Menu
pub async fn create_theme_menu(
    ctx: &Context,
    channel_id: ChannelId,
    mut anime_entries: Vec<Anime>,
    owner: UserId,
) -> BotResult<()> {
    let nsfw = ctx.http.get_channel(channel_id.0).await?.is_nsfw();
    anime_entries.sort_by_key(|a| {
        if let Some(theme) = a.themes.as_ref().and_then(|t| t.first()) {
            match &theme.theme_type {
                ThemeType::OP => theme.sequence.unwrap_or(1),
                ThemeType::ED => theme.sequence.unwrap_or(1) * 100,
            }
        } else {
            10000
        }
    });
    let pages = create_theme_pages(anime_entries, nsfw);
    MenuBuilder::new_paginator()
        .add_pages(pages)
        .timeout(EXTRA_LONG_TIMEOUT)
        .owner(owner)
        .build(ctx, channel_id)
        .await?;

    Ok(())
}

fn create_theme_pages(anime_entries: Vec<Anime>, nsfw: bool) -> Vec<Page<'static>> {
    let mut pages = Vec::new();
    for anime in anime_entries {
        if anime.themes.is_none() {
            continue;
        }
        for theme in anime.themes.unwrap() {
            if theme.entries.is_none() {
                continue;
            }
            let sequence = theme.sequence.clone().unwrap_or(1);
            for entry in theme.entries.unwrap() {
                if entry.nsfw && !nsfw {
                    continue;
                }
                let page = create_theme_page(&anime.name, &theme.theme_type, sequence, entry);
                pages.push(page);
            }
        }
    }
    if pages.is_empty() {
        let mut message = CreateMessage::default();
        message.embed(|e| e.description("No themes found!"));
        pages.push(Page::Static(message));
    }

    pages
}

/// Creates a new anime theme page
fn create_theme_page(
    anime_name: &str,
    theme_type: &ThemeType,
    theme_sequence: u16,
    entry: ThemeEntry,
) -> Page<'static> {
    let mut message = CreateMessage::default();
    let videos = entry.videos.unwrap();

    let theme_type = match theme_type {
        ThemeType::OP => "Opening",
        ThemeType::ED => "Ending",
    };

    message.content(format!(
        "**{} {}** of **{}**\nhttps://animethemes.moe/video/{}",
        theme_type,
        theme_sequence,
        anime_name,
        videos.first().unwrap().basename
    ));

    Page::Static(message)
}
