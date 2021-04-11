use regex::Regex;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;

use crate::providers::qalc;

static QALC_HELP: &[&str] = &["help", "--help", "-h", "h"];

#[command]
#[description("Calculates an expression")]
#[min_args(1)]
#[usage("<expression>")]
#[example("1 * 1 + 1 / sqrt(2)")]
#[bucket("general")]
async fn qalc(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let expression = args.message();
    lazy_static::lazy_static! {
        static ref ERROR_REGEX: Regex = Regex::new(r"^error:.*").unwrap();
    }

    if QALC_HELP.contains(&expression) {
        msg.channel_id.send_message(ctx, |f| {
            f.embed(|e| e.title("Help").description("Read the [Qalculate! Manual](https://qalculate.github.io/manual/index.html)"))
        }).await?;
    } else {
        let result = qalc::qalc(expression).await?;
        let mut description = format!("`{}`", result);

        if ERROR_REGEX.is_match(&result) {
            description +=
                "\nRead the [Qalculate! Manual](https://qalculate.github.io/manual/index.html)";
        }
        if &result == "aborted\n" {
            description =
                "Calculation aborted after timeout. Try a less complex calculation.".to_string();
        }

        msg.channel_id
            .send_message(ctx, |f| {
                f.embed(|e| {
                    e.description(description)
                        .footer(|f| f.text("Powered by qalculate!"))
                })
            })
            .await?;
    }

    Ok(())
}
