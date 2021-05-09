use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;
use serenity::model::id::UserId;

#[command]
#[description("Fuck this person in particular")]
#[usage("[<amount>] [<verbosity>]")]
#[min_args(1)]
#[max_args(3)]
#[bucket("general")]
#[aliases("frick", "fock")]
async fn fuck(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let person = args.single::<UserId>()?;
    let mut amount = args.single::<usize>().unwrap_or(3);
    if amount > 10 {
        amount = 10;
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
            .say(&ctx, format!("{} <@{}>", fuck_word, person))
            .await?;
    }

    Ok(())
}
