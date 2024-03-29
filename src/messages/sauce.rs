use std::cmp::Ordering;

use serenity::builder::CreateMessage;
use serenity::{model::channel::Message, prelude::*};

use bot_coreutils::url::get_domain_for_url;

use crate::utils::error::BotResult;
use rand::prelude::SliceRandom;
use serenity_additions::menu::{MenuBuilder, Page};
use std::time::Duration;

static MAX_RESULTS: usize = 6;
static MIN_SIMILARITY: f32 = 50.0;
static SAUCE_MESSAGES: &[&str] = &[
    "Here's your mayonnaise",
    "Here's your punched tomatoes",
    "Warning: It's tabasco",
    "Yum, apple sauce",
];

/// Builds a new sauce menu
pub async fn show_sauce_menu(
    ctx: &Context,
    msg: &Message,
    sources: Vec<sauce_api::source::Output>,
) -> BotResult<()> {
    let pages: Vec<Page> = sources.into_iter().map(create_sauce_page).collect();

    if pages.len() == 1 {
        MenuBuilder::default()
            .timeout(Duration::from_secs(600))
            .add_pages(pages)
            .build(ctx, msg.channel_id)
            .await?;
    } else {
        MenuBuilder::new_paginator()
            .timeout(Duration::from_secs(600))
            .add_pages(pages)
            .show_help()
            .build(ctx, msg.channel_id)
            .await?;
    }

    Ok(())
}

/// Creates a single sauce page
fn create_sauce_page<'a>(mut result: sauce_api::source::Output) -> Page<'a> {
    let mut message = CreateMessage::default();
    let mut description_lines = Vec::new();
    let original = result.original_url;
    description_lines.push(format!("[Original]({})", original));
    description_lines.push(String::new());
    // sort by similarity
    result.items.sort_by(|a, b| {
        if a.similarity > b.similarity {
            Ordering::Greater
        } else if a.similarity < b.similarity {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    });
    // display with descending similarity
    result.items.reverse();
    let items: Vec<(usize, sauce_api::source::Item)> = result
        .items
        .into_iter()
        .filter(|i| i.similarity >= MIN_SIMILARITY)
        .enumerate()
        .collect();

    if items.is_empty() {
        description_lines.push("*No Sources found*".to_string());
    }

    for (i, item) in items {
        if i >= MAX_RESULTS {
            break;
        }
        description_lines.push(format!(
            "{}% Similarity: [{}]({})",
            item.similarity,
            get_domain_for_url(&item.link).unwrap_or("Source".to_string()),
            item.link
        ));
    }
    let mut rng = rand::thread_rng();

    message.embed(|e| {
        e.title(SAUCE_MESSAGES.choose(&mut rng).unwrap())
            .description(description_lines.join("\n"))
            .thumbnail(original)
            .footer(|f| f.text("Powered by SauceNAO"))
    });

    Page::new_static(message)
}
