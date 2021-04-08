use rand::prelude::*;
use regex::Regex;
use serenity::framework::standard::{Args, CommandError, CommandResult};
use serenity::model::channel::Message;
use serenity::{framework::standard::macros::command, prelude::*};

// return a normal peko in most cases
static PEKOS: &[&str] = &[
    "peko",
    "p e k o",
    "peeeeekooooo",
    "pppeeekkkooo",
    "🇵 🇪 🇰 🇴",
    "p3k0",
];

#[command]
#[description("Pekofy messages")]
#[usage("(<content>)")]
#[example("Hello")]
#[aliases("peko")]
async fn pekofy(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut reference_message = msg.id;
    let mut content = args.message().to_string();

    if args.is_empty() {
        if let Some(reference) = &msg.referenced_message {
            reference_message = reference.id;
            content = reference.content.clone();
        } else {
            let messages = msg
                .channel_id
                .messages(ctx, |ret| ret.before(&msg.id).limit(1))
                .await?;
            let reference = messages
                .first()
                .ok_or(CommandError::from("No message to pekofy"))?;

            reference_message = reference.id;
            content = reference.content.clone();
        };
        let _ = msg.delete(ctx).await;
    }
    if content.is_empty() {
        return Err(CommandError::from("Can't pekofy empty message"));
    }

    log::debug!("Pekofying message '{}'", content);
    let mut alpha_lowercase = content.to_lowercase();
    alpha_lowercase.retain(|c| c.is_alphanumeric());

    let pekofied: String = if alpha_lowercase == "pain" {
        "https://tenor.com/view/pekora-usada-peko-hololive-died-gif-18114577".to_string()
    } else if PEKOS.contains(&&*alpha_lowercase) {
        random_peko()
    } else {
        content
            .lines()
            .into_iter()
            .map(pekofy_line)
            .collect::<Vec<String>>()
            .join("\n")
    };

    let message = ctx
        .http
        .get_message(msg.channel_id.0, reference_message.0)
        .await?;
    log::debug!("Pekofied message is '{}'", pekofied);
    message.reply(ctx, pekofied).await?;

    Ok(())
}

/// Pekofies a single line
fn pekofy_line(mut line: &str) -> String {
    lazy_static::lazy_static! { static ref FORMATTING_REGEX: Regex = Regex::new(r"^(.*?)((<:\w+:\d+>|\W)*)$").unwrap(); }
    log::debug!("Pekofying line '{}'", line);
    let original = line;

    let mut md = "";
    if let Some(captures) = FORMATTING_REGEX.captures(line) {
        line = captures.get(1).unwrap().as_str();
        md = captures.get(2).unwrap().as_str();
    }

    for peko in PEKOS {
        if line.to_lowercase().ends_with(peko) {
            log::debug!("Peko already found in message. Returning original");
            return original.to_string();
        }
    }

    let mut peko = random_peko();

    if line
        .chars()
        .filter(|c| c.is_alphabetic())
        .all(char::is_uppercase)
    {
        log::debug!("Message is all uppercase. Peko will also be uppercase");
        peko = peko.to_uppercase();
    }

    format!("{} {}{}", line, peko, md)
}

/// Returns a random peko
fn random_peko() -> String {
    let mut rng = rand::thread_rng();
    if rng.gen_range(0..20) == 10 {
        PEKOS.choose(&mut rng).unwrap().to_string()
    } else {
        "peko".to_string()
    }
}
