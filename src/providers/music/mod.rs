use crate::providers::music::responses::{PlaylistEntry, VideoInformation};
use crate::utils::error::BotResult;
use std::process::Stdio;
use tokio::io::AsyncReadExt;
use tokio::process::Command;

pub(crate) mod queue;
pub(crate) mod responses;
pub(crate) mod spotify;

/// Returns a list of youtube videos for a given url
pub(crate) async fn get_videos_for_playlist(url: &str) -> BotResult<Vec<PlaylistEntry>> {
    let output =
        youtube_dl(&["--no-warnings", "--flat-playlist", "--dump-json", "-i", url]).await?;

    let videos = output
        .lines()
        .filter_map(|l| serde_json::from_str::<PlaylistEntry>(l).ok())
        .collect();

    Ok(videos)
}

/// Returns information for a single video by using youtube-dl
pub(crate) async fn get_video_information(url: &str) -> BotResult<VideoInformation> {
    let output = youtube_dl(&["--no-warnings", "--dump-json", "-i", url]).await?;

    let information = serde_json::from_str(&*output)?;

    Ok(information)
}

/// Searches for a video
pub(crate) async fn search_video_information(query: String) -> BotResult<Option<VideoInformation>> {
    let output = youtube_dl(&[
        "--no-warnings",
        "--dump-json",
        "-i",
        format!("ytsearch:\"{}\"", query).as_str(),
    ])
    .await?;
    let information = serde_json::from_str(&*output)?;

    Ok(information)
}

/// Executes youtube-dl asynchronously
async fn youtube_dl(args: &[&str]) -> BotResult<String> {
    let ytdl = Command::new("youtube-dl")
        .args(args)
        .stdout(Stdio::piped())
        .spawn()?;
    let mut output = String::new();
    ytdl.stdout.unwrap().read_to_string(&mut output).await?;

    Ok(output)
}
