use crate::commands::music::utils::{get_channel_for_author, join_channel};
use crate::providers::ytdl::get_videos_for_url;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandError, CommandResult};
use serenity::model::channel::Message;

#[command]
#[only_in(guilds)]
#[description("Plays a song in a voice channel")]
#[usage("play <url>")]
#[min_args(1)]
#[max_args(1)]
#[aliases("p")]
async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let url = args.message();

    if !url.starts_with("http") {
        return Err(CommandError::from("The provided url is not valid"));
    }

    let guild = msg.guild(&ctx.cache).await.unwrap();

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let mut handler = manager.get(guild.id);

    if handler.is_none() {
        msg.guild(&ctx.cache).await.unwrap();
        let channel_id = get_channel_for_author(&msg.author.id, &guild)?;
        handler = Some(join_channel(ctx, channel_id, guild.id).await);
    }

    let handler_lock = handler.ok_or(CommandError::from("Not in a voice channel"))?;
    let mut handler = handler_lock.lock().await;
    let mut videos: Vec<String> = get_videos_for_url(url)?
        .into_iter()
        .map(|v| format!("https://www.youtube.com/watch?v={}", v.url))
        .collect();
    if videos.len() == 0 {
        videos.push(url.to_string());
    }

    let mut metadata = None;

    for video in &videos {
        let source = match songbird::ytdl(video).await {
            Ok(s) => s,
            Err(e) => {
                msg.channel_id
                    .say(ctx, format!("Failed to enqueue {}: {:?}", video, e))
                    .await?;
                continue;
            }
        };

        metadata = Some(source.metadata.clone());

        handler.enqueue_source(source);
    }
    if videos.len() == 1 {
        let metadata = metadata.unwrap();
        msg.channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|mut e| {
                    e = e.description(format!(
                        "Added [{}]({}) to the queue",
                        metadata.title.unwrap(),
                        url
                    ));
                    if let Some(thumb) = metadata.thumbnail {
                        e = e.thumbnail(thumb);
                    }

                    e
                })
            })
            .await?;
    } else {
        msg.channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|e| e.description(format!("Added {} songs to the queue", videos.len())))
            })
            .await?;
    }

    Ok(())
}
