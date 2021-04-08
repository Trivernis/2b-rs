use serenity::client::Context;
use serenity::framework::standard::{Args, CommandError, CommandResult};
use serenity::framework::standard::macros::command;
use serenity::model::channel::Message;

use crate::commands::music::{
    get_channel_for_author, get_queue_for_guild, get_songs_for_url, get_voice_manager,
    join_channel, play_next_in_queue,
};

#[command]
#[only_in(guilds)]
#[description("Puts a song as the next to play in the queue")]
#[usage("play_next <song-url>")]
#[min_args(1)]
#[max_args(2)]
#[aliases("pn")]
#[allowed_roles("DJ")]
async fn play_next(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let url = args.message();

    if !url.starts_with("http") {
        return Err(CommandError::from("The provided url is not valid"));
    }

    let guild = msg.guild(&ctx.cache).await.unwrap();

    let manager = get_voice_manager(ctx).await;
    let mut handler = manager.get(guild.id);

    if handler.is_none() {
        msg.guild(&ctx.cache).await.unwrap();
        let channel_id = get_channel_for_author(&msg.author.id, &guild)?;
        handler = Some(join_channel(ctx, channel_id, guild.id).await);
    }

    let handler = handler.ok_or(CommandError::from("Not in a voice channel"))?;

    let mut songs = get_songs_for_url(&ctx, msg, url).await?;

    let queue = get_queue_for_guild(ctx, &guild.id).await?;
    let play_first = {
        let mut queue_lock = queue.lock().await;
        songs.reverse();

        for song in songs {
            queue_lock.add_next(song);
        }
        queue_lock.current().is_none()
    };

    if play_first {
        play_next_in_queue(&ctx.http, &msg.channel_id, &queue, &handler).await;
    }

    Ok(())
}