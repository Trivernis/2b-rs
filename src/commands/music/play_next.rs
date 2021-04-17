use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandError, CommandResult};
use serenity::model::channel::Message;

use crate::commands::common::handle_autodelete;
use crate::commands::music::{
    get_channel_for_author, get_queue_for_guild, get_songs_for_query, get_voice_manager,
    join_channel, play_next_in_queue, DJ_CHECK,
};
use crate::messages::music::now_playing::create_now_playing_msg;

#[command]
#[only_in(guilds)]
#[description("Puts a song as the next to play in the queue")]
#[usage("(<spotify_ur>|<youtube_url>|<query>|pl:<saved_playlist>)")]
#[min_args(1)]
#[aliases("pn", "play-next", "playnext")]
#[bucket("music_api")]
#[checks(DJ)]
async fn play_next(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let query = args.message();

    let guild = msg.guild(&ctx.cache).await.unwrap();
    log::debug!("Playing song as next song for guild {}", guild.id);
    let manager = get_voice_manager(ctx).await;
    let mut handler = manager.get(guild.id);

    if handler.is_none() {
        log::debug!("Not in a voice channel. Joining authors channel");
        msg.guild(&ctx.cache).await.unwrap();
        let channel_id = get_channel_for_author(&msg.author.id, &guild)?;
        handler = Some(join_channel(ctx, channel_id, guild.id).await);
    }

    let handler = forward_error!(
        ctx,
        msg.channel_id,
        handler.ok_or(CommandError::from("I'm not in a voice channel"))
    );

    let mut songs = get_songs_for_query(&ctx, msg, query).await?;

    let queue = get_queue_for_guild(ctx, &guild.id).await?;
    let (play_first, create_now_playing) = {
        let mut queue_lock = queue.lock().await;
        songs.reverse();
        log::debug!("Enqueueing songs as next songs in the queue");

        for song in songs {
            queue_lock.add_next(song);
        }
        (
            queue_lock.current().is_none(),
            queue_lock.now_playing_msg.is_none(),
        )
    };

    if play_first {
        while !play_next_in_queue(&ctx.http, &msg.channel_id, &queue, &handler).await {}
    }
    if create_now_playing {
        let handle = create_now_playing_msg(ctx, queue.clone(), msg.channel_id).await?;
        let mut queue_lock = queue.lock().await;
        queue_lock.now_playing_msg = Some(handle);
    }
    handle_autodelete(ctx, msg).await?;

    Ok(())
}
