use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;
use std::time::Duration;

use futures::future::BoxFuture;
use futures::FutureExt;

use bot_coreutils::process::run_command_async;

use crate::providers::music::queue::Song;
use crate::providers::music::responses::{PlaylistEntry, VideoInformation};
use crate::utils::error::BotResult;
use bot_coreutils::string::enquote;

static THREAD_LIMIT: u8 = 64;

/// Returns a list of youtube videos for a given url
pub(crate) async fn get_videos_for_playlist(url: &str) -> BotResult<Vec<PlaylistEntry>> {
    tracing::debug!("Getting playlist information for {}", url);
    let output =
        youtube_dl(&["--no-warnings", "--flat-playlist", "--dump-json", "-i", url]).await?;

    let videos = output
        .lines()
        .filter_map(|l| serde_json::from_str::<PlaylistEntry>(l).ok())
        .collect();

    Ok(videos)
}

/// Searches for a video
pub(crate) async fn search_video_information(query: String) -> BotResult<Option<VideoInformation>> {
    tracing::debug!("Searching for video '{}'", query);
    let output = youtube_dl(&[
        "--no-warnings",
        "--dump-json",
        "-i",
        format!("ytsearch:{}", enquote(query)).as_str(),
    ])
    .await?;
    let information = serde_json::from_str(&*output)?;

    Ok(information)
}

/// Searches songs on youtube in parallel
#[allow(dead_code)]
async fn parallel_search_youtube(song_names: Vec<String>) -> Vec<Song> {
    let search_futures: Vec<BoxFuture<BotResult<Option<VideoInformation>>>> = song_names
        .into_iter()
        .map(|s| search_video_information(s).boxed())
        .collect();
    let information: Vec<BotResult<Option<VideoInformation>>> =
        futures::future::join_all(search_futures).await;
    information
        .into_iter()
        .filter_map(|i| i.ok().and_then(|s| s).map(Song::from))
        .collect()
}

/// Executes youtube-dl asynchronously
/// An atomic U8 is used to control the number of parallel processes
/// to avoid using too much memory
async fn youtube_dl(args: &[&str]) -> BotResult<String> {
    lazy_static::lazy_static! { static ref THREAD_LOCK: Arc<AtomicU8> = Arc::new(AtomicU8::new(0)); }
    tracing::trace!("Running youtube-dl with args {:?}", args);

    while THREAD_LOCK.load(Ordering::SeqCst) >= THREAD_LIMIT {
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    THREAD_LOCK.fetch_add(1, Ordering::Relaxed);

    let output = run_command_async("youtube-dl", args).await.map_err(|e| {
        THREAD_LOCK.fetch_sub(1, Ordering::Relaxed);
        e
    })?;
    tracing::trace!("youtube-dl response is {}", output);
    THREAD_LOCK.fetch_sub(1, Ordering::Relaxed);

    Ok(output)
}
