use chrono::prelude::*;
use chrono_tz::Tz;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;

#[command]
#[description("Converts a time into a different timezone")]
#[min_args(1)]
#[max_args(3)]
#[usage("<%H:%M/now> (<from-timezone>) (<to-timezone>)")]
#[bucket("general")]
async fn time(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let when = args.single::<String>().unwrap_or("now".to_string());
    let first_timezone = args.single::<String>().ok();
    let second_timezone = args.single::<String>().ok();

    let from_timezone: Tz = if let Some(first) = &first_timezone {
        first.parse::<Tz>()?
    } else {
        Tz::UTC
    };

    let to_timezone = if let Some(second) = &second_timezone {
        second.parse::<Tz>()?
    } else {
        Tz::UTC
    };

    let time = if when.to_lowercase() == "now" {
        Utc::now().with_timezone(&from_timezone)
    } else {
        let now = Utc::now();
        if second_timezone.is_some() {
            from_timezone.datetime_from_str(
                &format!("{} {}:00", now.format("%Y-%m-%d"), &*when),
                "%Y-%m-%d %H:%M:%S",
            )?
        } else {
            let timezone: Tz = "UTC".parse().unwrap();
            timezone
                .datetime_from_str(
                    &format!("{} {}:00", now.format("%Y-%m-%d"), &*when),
                    "%Y-%m-%d %H:%M:%S",
                )?
                .with_timezone(&from_timezone)
        }
    };

    if second_timezone.is_some() {
        msg.channel_id
            .say(
                ctx,
                format!(
                    "{} is {}",
                    time.format("%H:%M %Z"),
                    time.with_timezone(&to_timezone).format("%H:%M %Z"),
                ),
            )
            .await?;
    } else {
        msg.channel_id
            .say(ctx, format!("{}", time.format("%H:%M %Z")))
            .await?;
    }

    Ok(())
}
