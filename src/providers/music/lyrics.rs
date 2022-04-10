use regex::Regex;
use serde_derive::Deserialize;

use crate::utils::error::BotResult;

const API_ENDPOINT: &str = "https://api.lyrics.ovh/v1/";

/// Returns the lyrics of a song
pub async fn get_lyrics(artist: &str, title: &str) -> BotResult<Option<String>> {
    lazy_static::lazy_static! { static ref DOUBLE_LB_REGEX: Regex = Regex::new(r"\n\n").unwrap(); }
    tracing::debug!("Requesting lyrics for '{}' by '{}'", title, artist);
    let request_url = format!("{}{}/{}", API_ENDPOINT, artist, title);
    tracing::trace!("Request url is {}", request_url);
    let response = reqwest::get(request_url).await?;
    let response_text = response.text().await?;
    tracing::trace!("Lyrics Response is {}", response_text);

    let lyrics: Option<Lyrics> = serde_json::from_str(&*response_text).ok();

    Ok(lyrics.map(|l| DOUBLE_LB_REGEX.replace_all(&*l.lyrics, "\n").to_string()))
}

#[derive(Deserialize, Clone, Debug)]
struct Lyrics {
    lyrics: String,
}
