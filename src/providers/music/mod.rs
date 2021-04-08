use std::io::Read;
use std::process::{Command, Stdio};

use crate::providers::music::responses::{PlaylistEntry, VideoInformation};
use crate::utils::error::BotResult;

pub(crate) mod queue;
pub(crate) mod responses;

/// Returns a list of youtube videos for a given url
pub(crate) fn get_videos_for_playlist(url: &str) -> BotResult<Vec<PlaylistEntry>> {
    let ytdl = Command::new("youtube-dl")
        .args(&["--no-warnings", "--flat-playlist", "--dump-json", "-i", url])
        .stdout(Stdio::piped())
        .spawn()?;

    let mut output = String::new();
    ytdl.stdout.unwrap().read_to_string(&mut output)?;

    let videos = output
        .lines()
        .filter_map(|l| serde_json::from_str::<PlaylistEntry>(l).ok())
        .collect();

    Ok(videos)
}

/// Returns information for a single video by using youtube-dl
pub(crate) fn get_video_information(url: &str) -> BotResult<VideoInformation> {
    let ytdl = Command::new("youtube-dl")
        .args(&["--no-warnings", "--dump-json", "-i", url])
        .stdout(Stdio::piped())
        .spawn()?;

    let information = serde_json::from_reader(ytdl.stdout.unwrap())?;

    Ok(information)
}

/// Searches for a video
pub(crate) fn search_video_information(query: &str) -> BotResult<Option<VideoInformation>> {
    let ytdl = Command::new("youtube-dl")
        .args(&[
            "--no-warnings",
            "--dump-json",
            "-i",
            format!("ytsearch:\"{}\"", query).as_str(),
        ])
        .stdout(Stdio::piped())
        .spawn()?;

    let information = serde_json::from_reader(ytdl.stdout.unwrap())?;

    Ok(information)
}
