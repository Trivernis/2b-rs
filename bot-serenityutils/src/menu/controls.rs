use crate::error::{SerenityUtilsError, SerenityUtilsResult};
use crate::menu::container::get_listeners_from_context;
use crate::menu::menu::Menu;
use crate::menu::typedata::HelpActiveContainer;
use crate::menu::ActionContainer;
use serde_json::json;
use serde_json::Value;
use serenity::client::Context;
use serenity::http::CacheHttp;
use serenity::model::channel::Reaction;
use std::sync::atomic::Ordering;

/// Shows the next page in the menu
pub async fn next_page(ctx: &Context, menu: &mut Menu<'_>, _: Reaction) -> SerenityUtilsResult<()> {
    log::debug!("Showing next page");
    menu.current_page = (menu.current_page + 1) % menu.pages.len();
    display_page(ctx, menu).await?;

    Ok(())
}

/// Shows the previous page in the menu
pub async fn previous_page(
    ctx: &Context,
    menu: &mut Menu<'_>,
    _: Reaction,
) -> SerenityUtilsResult<()> {
    log::debug!("Showing previous page");
    if menu.current_page == 0 {
        menu.current_page = menu.pages.len() - 1;
    } else {
        menu.current_page = menu.current_page - 1;
    }
    display_page(ctx, menu).await?;

    Ok(())
}

/// Shows the previous page in the menu
pub async fn close_menu(
    ctx: &Context,
    menu: &mut Menu<'_>,
    _: Reaction,
) -> SerenityUtilsResult<()> {
    log::debug!("Closing menu");
    menu.close(ctx.http()).await?;
    let listeners = get_listeners_from_context(&ctx).await?;
    let mut listeners_lock = listeners.lock().await;
    let message = menu.message.read().await;
    listeners_lock.remove(&*message);

    Ok(())
}

pub async fn toggle_help(
    ctx: &Context,
    menu: &mut Menu<'_>,
    _: Reaction,
) -> SerenityUtilsResult<()> {
    log::debug!("Displaying help");
    let show_help = menu
        .data
        .get::<HelpActiveContainer>()
        .expect("Missing HelpActiveContainer in menu data")
        .clone();

    if show_help.load(Ordering::Relaxed) {
        display_page(ctx, menu).await?;
        show_help.store(false, Ordering::Relaxed);
        return Ok(());
    }
    let page = menu
        .pages
        .get(menu.current_page)
        .ok_or(SerenityUtilsError::PageNotFound(menu.current_page))?
        .get()
        .await?;
    let mut message = menu.get_message(ctx.http()).await?;
    log::debug!("Building help entries");
    let mut help_entries = menu
        .help_entries
        .iter()
        .filter_map(|(e, h)| Some((menu.controls.get(e)?, e, h)))
        .collect::<Vec<(&ActionContainer, &String, &String)>>();
    help_entries.sort_by_key(|(c, _, _)| c.position());
    let help_message = help_entries
        .into_iter()
        .map(|(_, e, h)| format!(" - {} {}", e, h))
        .collect::<Vec<String>>()
        .join("\n");
    log::trace!("Help message is {}", help_message);

    message
        .edit(ctx, |m| {
            m.0.clone_from(&mut page.0.clone());

            if let Some(embed) = m.0.get_mut("embed") {
                let embed = embed.as_object_mut().unwrap();
                let fields = embed
                    .entry("fields")
                    .or_insert_with(|| Value::Array(vec![]));
                if let Value::Array(ref mut inner) = *fields {
                    inner.push(json!({
                        "inline": false,
                        "name": "Help".to_string(),
                        "value": help_message,
                    }));
                }
            } else {
                m.embed(|e| {
                    e.field("Help", help_message, false);

                    e
                });
            }

            m
        })
        .await?;
    log::debug!("Help message displayed");
    show_help.store(true, Ordering::Relaxed);

    Ok(())
}

/// Displays the menu page
async fn display_page(ctx: &Context, menu: &mut Menu<'_>) -> SerenityUtilsResult<()> {
    log::debug!("Displaying page {}", menu.current_page);
    let page = menu
        .pages
        .get(menu.current_page)
        .ok_or(SerenityUtilsError::PageNotFound(menu.current_page))?
        .get()
        .await?;
    let mut msg = menu.get_message(ctx.http()).await?;

    msg.edit(ctx, |e| {
        e.0.clone_from(&mut page.0.clone());
        e
    })
    .await?;
    log::debug!("Page displayed");

    Ok(())
}
