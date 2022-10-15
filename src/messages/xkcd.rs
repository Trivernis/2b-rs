use crate::utils::error::BotResult;
use serenity::builder::CreateMessage;
use serenity::client::Context;
use serenity::model::id::{ChannelId, UserId};
use serenity_additions::core::LONG_TIMEOUT;
use serenity_additions::menu::{MenuBuilder, Page};
use xkcd_search::Comic;

/// Creates a new xkcd menu
pub async fn create_xkcd_menu(
    ctx: &Context,
    channel_id: ChannelId,
    comics: Vec<Comic>,
    owner: UserId,
) -> BotResult<()> {
    let mut builder = if comics.len() > 1 {
        MenuBuilder::new_paginator()
    } else {
        MenuBuilder::default()
    };
    if comics.is_empty() {
        let mut message = CreateMessage::default();
        message.content("No Comics found");
        builder = builder.add_page(Page::new_static(message));
    }
    builder
        .add_pages(comics.into_iter().map(|c| create_xkcd_page(c)))
        .timeout(LONG_TIMEOUT)
        .owner(owner)
        .build(ctx, channel_id)
        .await?;

    Ok(())
}

/// Creates an xkcd message
fn create_xkcd_page<'a>(comic: Comic) -> Page<'a> {
    let mut message = CreateMessage::default();

    message.embed(|e| {
        e.title(format!("#{} - {}", comic.num, comic.title))
            .image(&comic.img)
            .url(format!("https://xkcd.com/{}", comic.num))
            .footer(|f| f.text(format!("{} | xkcd.com", comic.alt)))
    });

    Page::new_static(message)
}
