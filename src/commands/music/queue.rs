use std::cmp::min;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

use crate::commands::music::get_queue_for_guild;

#[command]
#[only_in(guilds)]
#[description("Shows the song queue")]
#[usage("queue")]
#[aliases("q")]
async fn queue(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();

    let queue = get_queue_for_guild(ctx, &guild.id).await?;
    let queue_lock = queue.lock().await;
    let songs: Vec<(usize, String)> = queue_lock
        .entries()
        .into_iter()
        .map(|s| s.title().clone())
        .enumerate()
        .collect();

    if songs.len() == 0 {
        msg.channel_id
            .send_message(ctx, |m| {
                m.embed(|e| e.title("Queue").description("*The queue is empty*"))
            })
            .await?;

        return Ok(());
    }

    let mut song_list = Vec::new();

    for i in 0..min(10, songs.len() - 1) {
        song_list.push(format!("{:0>3} - {}", songs[i].0, songs[i].1))
    }
    if songs.len() > 10 {
        song_list.push("...".to_string());
        let last = songs.last().unwrap();
        song_list.push(format!("{:0>3} - {}", last.0, last.1))
    }
    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| {
                e.title("Queue")
                    .description(format!("```\n{}\n```", song_list.join("\n")))
            })
        })
        .await?;

    Ok(())
}
