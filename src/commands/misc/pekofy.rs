use serenity::framework::standard::{Args, CommandError, CommandResult};
use serenity::model::channel::Message;
use serenity::{framework::standard::macros::command, prelude::*};

static MESSAGE_DELIMITERS: &[char] = &['.', '?', '!', '"'];
static MARKDOWN_SPECIAL_CHARACTERS: &[&str] = &["~~", "**", "*"];

#[command]
#[description("Pekofy messages")]
#[usage("(<content>)")]
#[example("Hello")]
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
    let pekofied: String = content
        .lines()
        .into_iter()
        .map(pekofy_line)
        .collect::<Vec<String>>()
        .join("\n");
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
    log::debug!("Pekofying line '{}'", line);
    let original = line;
    let mut md_index = None;

    for pattern in MARKDOWN_SPECIAL_CHARACTERS {
        if let Some(i) = line.rfind(pattern) {
            log::debug!("Found markdown at index {}", i);
            md_index = Some(i);
            break;
        }
    }
    let mut md = "";
    if let Some(index) = md_index {
        let (line_part, md_part) = line.split_at(index);
        line = line_part;
        md = md_part;
    }
    if line.ends_with("peko") {
        log::debug!("Peko already found in message. Returning original");
        return original.to_string();
    }

    let punctuation_index = line.rfind(MESSAGE_DELIMITERS);
    let mut peko = "peko".to_string();

    if line
        .chars()
        .filter(|c| c.is_alphabetic())
        .all(char::is_uppercase)
    {
        log::debug!("Message is all uppercase. Peko will also be uppercase");
        peko = peko.to_uppercase();
    }

    if let Some(index) = punctuation_index {
        log::debug!("Found punctuation at index {}", index);
        let (before, after) = line.split_at(index);
        format!("{} {}{}{}", before, peko, after, md)
    } else {
        format!("{} {}{}", line, peko, md)
    }
}
