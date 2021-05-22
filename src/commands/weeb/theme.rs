use crate::messages::theme::create_theme_menu;
use animethemes_rs::client::AnimeThemesClient;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;
use serenity_rich_interaction::core::MEDIUM_TIMEOUT;
use serenity_rich_interaction::ephemeral_message::EphemeralMessage;

#[command]
#[description("Query for the opening/ending/insert song of an anime")]
#[usage("<query..>")]
#[aliases("animetheme", "anime-theme", "opening", "ending", "ost")]
#[bucket("general")]
async fn theme(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let query = args.message();
    let client = AnimeThemesClient::default();
    let themes = client
        .search(query, &["entries"], &["theme", "theme.anime", "videos"])
        .await?;
    if let Some(entries) = themes.entries {
        if entries.is_empty() {
            EphemeralMessage::create(&ctx.http, msg.channel_id, MEDIUM_TIMEOUT, |c| {
                c.reference_message(msg).content("No themes found")
            })
            .await?;
        } else {
            create_theme_menu(ctx, msg.channel_id, entries).await?;
        }
    } else {
        EphemeralMessage::create(&ctx.http, msg.channel_id, MEDIUM_TIMEOUT, |c| {
            c.reference_message(msg).content("No themes found")
        })
        .await?;
    }

    Ok(())
}
