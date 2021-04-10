use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

use crate::commands::common::handle_autodelete;
use crate::utils::context_data::get_database_from_context;

#[command]
#[only_in(guilds)]
#[description("Displays a list of all saved playlists")]
#[usage("")]
async fn playlists(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    log::debug!("Displaying playlists for guild {}", guild.id);
    let database = get_database_from_context(ctx).await;

    let playlists = database.get_guild_playlists(guild.id.0).await?;
    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title("Saved Playlists").description(
                    playlists
                        .into_iter()
                        .map(|p| format!("[{}]({})", p.name, p.url))
                        .collect::<Vec<String>>()
                        .join("\n"),
                )
            })
        })
        .await?;
    handle_autodelete(ctx, msg).await?;

    Ok(())
}
