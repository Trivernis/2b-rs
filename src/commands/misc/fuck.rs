use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;
use serenity::model::id::UserId;
use std::time::Duration;

#[command]
#[description("Fuck this person in particular")]
#[usage("<person> [<amount>] [<verbosity>]")]
#[min_args(1)]
#[max_args(3)]
#[bucket("general")]
#[aliases("frick", "fock")]
async fn fuck(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let person = args.single::<UserId>()?;
    let mut amount = args.single::<usize>().unwrap_or(3);
    if amount > 3 {
        msg.reply(&ctx.http, "Don't you think that's a bit much?")
            .await?;
        tokio::time::sleep(Duration::from_secs(2)).await;
        amount = 3;
    } else {
        msg.reply(&ctx.http, "no").await?;
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
    let mut verbosity = args.single::<usize>().unwrap_or(1);
    if verbosity == 0 {
        verbosity = 1
    }
    let fuck_word = match verbosity {
        1 => "frick",
        2 => "flock",
        3 => "fock",
        4 => "fck",
        _ => "fuck",
    };

    for _ in 0..amount {
        msg.channel_id
            .say(&ctx, format!("{} <@{}>", fuck_word, msg.author.id))
            .await?;
    }

    Ok(())
}
