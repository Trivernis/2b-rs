use std::cmp::Ordering;

use sauce_api::{SauceItem, SauceResult};
use serenity::builder::CreateMessage;
use serenity::{model::channel::Message, prelude::*};

use bot_coreutils::url::get_domain_for_url;

use crate::utils::error::BotResult;
use bot_serenityutils::menu::MenuBuilder;
use std::time::Duration;

static MAX_RESULTS: usize = 6;
static MIN_SIMILARITY: f32 = 50.0;

/// Builds a new sauce menu
pub async fn show_sauce_menu(
    ctx: &Context,
    msg: &Message,
    sources: Vec<SauceResult>,
) -> BotResult<()> {
    let pages: Vec<CreateMessage> = sources.into_iter().map(create_sauce_page).collect();

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
            .build(ctx, msg.channel_id)
            .await?;
    };

    Ok(())
}

/// Creates a single sauce page
fn create_sauce_page<'a>(mut result: SauceResult) -> CreateMessage<'a> {
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
    let items: Vec<(usize, SauceItem)> = result
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

    message.embed(|e| {
        e.title("Sources")
            .description(description_lines.join("\n"))
            .thumbnail(original)
            .footer(|f| f.text("Powered by SauceNAO"))
    });

    message
}
