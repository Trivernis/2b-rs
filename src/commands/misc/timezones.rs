use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;

#[command]
#[description("Searches for timezones matching the query")]
#[min_args(1)]
#[usage("<query...>")]
#[example("Europe Berlin")]
#[bucket("general")]
async fn timezones(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let query = args
        .iter::<String>()
        .map(|s| s.unwrap().to_lowercase())
        .collect::<Vec<String>>();
    let mut variants: Vec<String> = chrono_tz::TZ_VARIANTS
        .iter()
        .map(|t| t.to_string())
        .filter(|name| query.iter().all(|q| name.to_lowercase().contains(q)))
        .collect();
    if variants.len() > 20 {
        let remaining = variants.len() - 20;
        variants = variants[0..20].to_vec();
        variants.push(format!("*and {} more...*", remaining));
    }
    let mut variants = variants.join("\n");

    if variants.is_empty() {
        variants = "*nothing found*".to_string();
    }
    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|e| e.title("Available Timezones").description(variants))
        })
        .await?;

    Ok(())
}
