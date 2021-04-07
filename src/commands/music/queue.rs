use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{CommandError, CommandResult};
use serenity::model::channel::Message;
use std::cmp::min;

#[command]
#[only_in(guilds)]
#[description("Shows the song queue")]
#[usage("queue")]
#[aliases("q")]
async fn queue(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let handler_lock = manager
        .get(guild.id)
        .ok_or(CommandError::from("Not in a voice channel"))?;
    let handler = handler_lock.lock().await;
    let songs: Vec<(usize, String)> = handler
        .queue()
        .current_queue()
        .into_iter()
        .map(|t| t.metadata().title.clone().unwrap())
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
