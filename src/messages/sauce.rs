use crate::utils::error::BotResult;
use crate::utils::get_domain_for_url;
use sauce_api::SauceResult;
use serenity::builder::CreateMessage;
use serenity::{model::channel::Message, prelude::*};
use serenity_utils::prelude::*;

/// Builds a new sauce menu
pub async fn show_sauce_menu(
    ctx: &Context,
    msg: &Message,
    sources: Vec<SauceResult>,
) -> BotResult<()> {
    let pages: Vec<CreateMessage> = sources.into_iter().map(create_sauce_page).collect();

    let menu = if pages.len() == 1 {
        Menu::new(
            ctx,
            msg,
            &pages,
            MenuOptions {
                controls: vec![],
                ..Default::default()
            },
        )
    } else {
        Menu::new(ctx, msg, &pages, MenuOptions::default())
    };
    menu.run().await?;

    Ok(())
}

/// Creates a single sauce page
fn create_sauce_page<'a>(result: SauceResult) -> CreateMessage<'a> {
    let mut message = CreateMessage::default();
    let mut description_lines = Vec::new();
    let original = result.original_url;
    description_lines.push(format!("[Original]({})", original));
    description_lines.push(String::new());

    for item in result.items {
        if item.similarity > 70. {
            description_lines.push(format!(
                "{}% Similarity: [{}]({})",
                item.similarity,
                get_domain_for_url(&item.link).unwrap_or("Source".to_string()),
                item.link
            ));
        }
    }
    message.embed(|e| {
        e.title("Sources")
            .description(description_lines.join("\n"))
            .thumbnail(original)
            .footer(|f| f.text("Powered by SauceNAO"))
    });

    message
}
