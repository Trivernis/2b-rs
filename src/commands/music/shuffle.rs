use std::collections::VecDeque;

use rand::Rng;
use serenity::client::Context;
use serenity::framework::standard::{CommandError, CommandResult};
use serenity::framework::standard::macros::command;
use serenity::model::channel::Message;

#[command]
#[only_in(guilds)]
#[description("Shuffles the queue")]
#[usage("shuffle")]
#[aliases("sh")]
async fn shuffle(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let handler_lock = manager
        .get(guild.id)
        .ok_or(CommandError::from("Not in a voice channel"))?;
    let handler = handler_lock.lock().await;
    handler.queue().modify_queue(shuffle_vec_deque);
    msg.channel_id
        .say(ctx, "The queue has been shuffled")
        .await?;

    Ok(())
}

/// Fisher-Yates shuffle for VecDeque
fn shuffle_vec_deque<T>(deque: &mut VecDeque<T>) {
    let mut rng = rand::thread_rng();
    let mut i = deque.len();
    while i >= 2 {
        i -= 1;
        deque.swap(i, rng.gen_range(0..i + 1))
    }
}
