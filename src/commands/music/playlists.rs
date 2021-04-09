use crate::utils::context_data::get_database_from_context;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

#[command]
#[only_in(guilds)]
#[description("Displays a list of all saved playlists")]
#[usage("")]
async fn playlists(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    log::debug!("Displaying playlists for guild {}", guild.id);
    let database = get_database_from_context(ctx).await;

    let playlists = database.get_guild_playlists(guild.id.0)?;
    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title("Saved Playlists")
                    .fields(playlists.into_iter().map(|p| (p.name, p.url, true)))
            })
        })
        .await?;

    Ok(())
}
