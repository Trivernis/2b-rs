use crate::utils::error::BotResult;
use rand::Rng;
use regex::Regex;
use serenity::client::Context;
use serenity::model::channel::Message;
use std::collections::VecDeque;

pub(crate) mod context_data;
pub(crate) mod error;
pub(crate) mod logging;
pub(crate) mod messages;
pub(crate) mod process;

/// Fisher-Yates shuffle for VecDeque
pub fn shuffle_vec_deque<T>(deque: &mut VecDeque<T>) {
    let mut rng = rand::thread_rng();
    let mut i = deque.len();
    while i >= 2 {
        i -= 1;
        deque.swap(i, rng.gen_range(0..i + 1))
    }
}

/// Returns the message the given message is a reply to or the message sent before that
pub async fn get_previous_message_or_reply(
    ctx: &Context,
    msg: &Message,
) -> BotResult<Option<Message>> {
    let referenced = if let Some(reference) = &msg.referenced_message {
        Some(*reference.clone())
    } else {
        let messages = msg
            .channel_id
            .messages(ctx, |ret| ret.before(&msg.id).limit(1))
            .await?;
        messages.first().cloned()
    };

    Ok(referenced)
}

/// Returns the domain for a given url
pub fn get_domain_for_url(url: &str) -> Option<String> {
    lazy_static::lazy_static! {
        static ref DOMAIN_REGEX: Regex = Regex::new(r"^(https?://)?(www\.)?((\w+\.)+\w+).*$").unwrap();
    }

    let captures = DOMAIN_REGEX.captures(url)?;

    captures.get(3).map(|c| c.as_str().to_string())
}

/// Returns the file for a given domain
pub fn get_file_name_for_domain(url: &str) -> Option<String> {
    lazy_static::lazy_static! {
        static ref FILE_REGEX: Regex = Regex::new(r"^(https?://)?(www\.)?(\w+\.)+\w+([^/]*/)*([^/]+)$").unwrap();
    }

    let captures = FILE_REGEX.captures(url)?;

    captures.get(3).map(|c| c.as_str().to_string())
}

/// Returns if the given file is an image
#[inline]
pub fn is_image(url: &str) -> bool {
    static IMAGE_EXTENSIONS: &[&str] = &["png", "webp", "jpg", "jpeg", "ico", "gif"];
    for ext in IMAGE_EXTENSIONS {
        if url.ends_with(ext) {
            return true;
        }
    }
    return false;
}

/// Returns if the given file is an image
#[inline]
pub fn is_video(url: &str) -> bool {
    static IMAGE_EXTENSIONS: &[&str] = &["mp4", "flv", "mkv", "webm"];
    for ext in IMAGE_EXTENSIONS {
        if url.ends_with(ext) {
            return true;
        }
    }
    return false;
}
