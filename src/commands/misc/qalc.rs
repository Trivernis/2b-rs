use crate::providers::qalc;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;

#[command]
#[description("Calculates an expression")]
#[min_args(1)]
#[usage("<expression>")]
#[example("1 * 1 + 1 / sqrt(2)")]
async fn qalc(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let expression = args.message();
    let result = qalc::qalc(expression).await?;
    msg.reply(ctx, format!("`{}`", result)).await?;

    Ok(())
}
