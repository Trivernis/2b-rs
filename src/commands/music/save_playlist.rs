use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;

use crate::commands::music::is_dj;
use crate::utils::context_data::get_database_from_context;

#[command]
#[only_in(guilds)]
#[description("Adds a playlist to the guilds saved playlists")]
#[usage("<name> <url/query>")]
#[example("anime https://www.youtube.com/playlist?list=PLqaM77H_o5hykROCe3uluvZEaPo6bZj-C")]
#[min_args(2)]
#[aliases("add-playlist", "save-playlist")]
#[bucket("general")]
async fn save_playlist(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();

    if !is_dj(ctx, guild.id, &msg.author).await? {
        msg.channel_id.say(ctx, "Requires DJ permissions").await?;
        return Ok(());
    }
    let name: String = args.single().unwrap();
    let url: &str = args.remains().unwrap();
    log::debug!(
        "Adding playlist '{}' with url '{}' to guild {}",
        name,
        url,
        guild.id
    );
    let database = get_database_from_context(ctx).await;

    database
        .add_guild_playlist(guild.id.0, name.clone(), url.to_string())
        .await?;

    msg.channel_id
        .say(ctx, format!("Playlist **{}** saved", name))
        .await?;

    Ok(())
}
