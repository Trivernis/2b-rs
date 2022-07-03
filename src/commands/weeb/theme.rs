use crate::messages::theme::create_theme_menu;
use animethemes_rs::client::AnimeThemesClient;
use animethemes_rs::includes::{AnimeInclude, SearchIncludes};
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;
use serenity_rich_interaction::core::MEDIUM_TIMEOUT;
use serenity_rich_interaction::ephemeral_message::EphemeralMessage;

#[command]
#[description("Query for the opening/ending/insert song of an anime")]
#[usage("<query..>")]
#[min_args(1)]
#[aliases("animetheme", "anime-theme", "opening", "ending", "ost")]
#[bucket("general")]
async fn theme(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let query = args.message();
    let client = AnimeThemesClient::default();
    let search_results = client
        .search(
            query,
            &["anime"],
            SearchIncludes {
                anime: AnimeInclude::default()
                    .themes()
                    .themes_entries()
                    .themes_entries_videos(),
                ..Default::default()
            },
        )
        .await?;
    if let Some(anime) = search_results.anime {
        if anime.is_empty() {
            EphemeralMessage::create(&ctx.http, msg.channel_id, MEDIUM_TIMEOUT, |c| {
                c.reference_message(msg).content("No themes found")
            })
            .await?;
        } else {
            create_theme_menu(ctx, msg.channel_id, anime, msg.author.id).await?;
        }
    } else {
        EphemeralMessage::create(&ctx.http, msg.channel_id, MEDIUM_TIMEOUT, |c| {
            c.reference_message(msg).content("No themes found")
        })
        .await?;
    }

    Ok(())
}
