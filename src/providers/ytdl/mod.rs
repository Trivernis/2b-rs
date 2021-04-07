use crate::providers::ytdl::playlist_entry::PlaylistEntry;
use crate::utils::error::{BotError, BotResult};
use std::io::Read;
use std::process::{Command, Stdio};

mod playlist_entry;

/// Returns a list of youtube videos for a given url
pub(crate) fn get_videos_for_url(url: &str) -> BotResult<Vec<PlaylistEntry>> {
    let ytdl = Command::new("youtube-dl")
        .args(&[
            "-f",
            "--no-warnings",
            "--flat-playlist",
            "--dump-json",
            "-i",
            url,
        ])
        .stdout(Stdio::piped())
        .spawn()?;

    let mut output = String::new();
    ytdl.stdout.unwrap().read_to_string(&mut output)?;

    let videos = output
        .lines()
        .map(|l| serde_json::from_str::<PlaylistEntry>(l).unwrap())
        .collect();

    Ok(videos)
}
