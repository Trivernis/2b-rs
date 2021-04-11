use crate::error::{SerenityUtilsError, SerenityUtilsResult};
use crate::menu::container::get_listeners_from_context;
use crate::menu::menu::Menu;
use serenity::client::Context;
use serenity::http::CacheHttp;
use serenity::model::channel::Reaction;

/// Shows the next page in the menu
pub async fn next_page(ctx: &Context, menu: &mut Menu<'_>, _: Reaction) -> SerenityUtilsResult<()> {
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
    menu.close(ctx.http()).await?;
    let listeners = get_listeners_from_context(&ctx).await?;
    let mut listeners_lock = listeners.lock().await;
    let message = menu.message.read().await;
    listeners_lock.remove(&*message);

    Ok(())
}

/// Displays the menu page
async fn display_page(ctx: &Context, menu: &mut Menu<'_>) -> SerenityUtilsResult<()> {
    let page = menu
        .pages
        .get(menu.current_page)
        .ok_or(SerenityUtilsError::PageNotFound(menu.current_page))?;
    let mut msg = menu.get_message(ctx.http()).await?;

    msg.edit(ctx, |e| {
        e.0.clone_from(&mut page.0.clone());
        e
    })
    .await?;

    Ok(())
}
