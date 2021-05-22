use crate::utils::error::BotResult;
use animethemes_rs::models::{ThemeEntry, ThemeType};
use serenity::builder::CreateMessage;
use serenity::client::Context;
use serenity::model::id::ChannelId;
use serenity_rich_interaction::core::EXTRA_LONG_TIMEOUT;
use serenity_rich_interaction::menu::{MenuBuilder, Page};

/// Creates a new Anime Theme Menu
pub async fn create_theme_menu(
    ctx: &Context,
    channel_id: ChannelId,
    mut entries: Vec<ThemeEntry>,
) -> BotResult<()> {
    let nsfw = ctx.http.get_channel(channel_id.0).await?.is_nsfw();
    entries.sort_by_key(|t| {
        if let Some(theme) = &t.theme {
            match &theme.theme_type {
                ThemeType::OP => theme.sequence.unwrap_or(1),
                ThemeType::ED => theme.sequence.unwrap_or(1) * 100,
            }
        } else {
            10000
        }
    });
    MenuBuilder::new_paginator()
        .add_pages(
            entries
                .into_iter()
                .filter(|e| {
                    if !nsfw && e.nsfw {
                        return false;
                    }
                    if let Some(videos) = &e.videos {
                        !videos.is_empty()
                    } else {
                        false
                    }
                })
                .map(|e| create_theme_page(e, nsfw)),
        )
        .timeout(EXTRA_LONG_TIMEOUT)
        .build(ctx, channel_id)
        .await?;

    Ok(())
}

/// Creates a new anime theme page
fn create_theme_page(entry: ThemeEntry, nsfw: bool) -> Page<'static> {
    let mut message = CreateMessage::default();
    let videos = entry.videos.unwrap();
    let theme = entry.theme.unwrap();
    let anime = theme.anime.unwrap();
    let theme_type = match theme.theme_type {
        ThemeType::OP => "Opening",
        ThemeType::ED => "Ending",
    };

    message.content(format!(
        "**{} {}** of **{}**\nhttps://animethemes.moe/video/{}",
        theme_type,
        theme.sequence.unwrap_or(1),
        anime.name,
        videos.first().unwrap().basename
    ));

    Page::Static(message)
}
