use crate::providers::music::queue::{Song, SongSource};
use crate::utils::context_data::StoreData;
use crate::utils::error::BotResult;
use aspotify::{ArtistSimplified, Track};
use bot_database::Database;
use regex::Regex;
use responses::VideoInformation;
use youtube_dl::search_video_information;

pub(crate) mod lyrics;
pub(crate) mod queue;
pub(crate) mod responses;
pub(crate) mod spotify;
pub(crate) mod youtube_dl;

/// Searches for a youtube video for the specified song
pub(crate) async fn song_to_youtube_video(song: &Song) -> BotResult<Option<VideoInformation>> {
    let artist = song.author().clone();
    let title = song.title().clone();
    let match_query = format!("{} - {}", artist, title);

    let queries = vec![
        format! {"{} - {} topic", artist, title},
        format!("{} - {} lyrics", artist, title),
        format!("{} - {} audio only", artist, title),
        format!("{} by {}", title, artist),
        match_query.clone(),
    ];

    let mut last_result = None;
    for query in queries {
        let result = search_video_information(query).await?;

        if let Some(video) = result {
            if trigram::similarity(&video.title, &match_query) >= 0.4
                || (trigram::similarity(&video.title, &title) >= 0.3
                    && trigram::similarity(&video.uploader, &artist) >= 0.3)
            {
                return Ok(Some(video));
            }
            last_result = Some(video);
        }
    }

    Ok(last_result)
}

/// Adds a youtube song to the database of songs
pub async fn add_youtube_song_to_database(
    store: &StoreData,
    database: &Database,
    song: &mut Song,
) -> BotResult<()> {
    let track = match song.source() {
        SongSource::Spotify(track) => track.clone(),
        SongSource::YouTube(_) => match search_for_song_variations(store, song).await {
            Ok(Some(track)) => track,
            Err(e) => {
                log::error!("Failed to search for song on spotify {:?}", e);
                return Ok(());
            }
            _ => return Ok(()),
        },
    };
    log::debug!("Song found on spotify. Inserting metadata");
    let artists = artists_to_string(track.artists);
    let url = song.url().await.unwrap();

    if let Some(id) = track.id {
        database
            .add_song(&id, &artists, &track.name, &track.album.name, &url)
            .await?;
    }

    Ok(())
}

/// Searches for multiple queries on spotify
async fn search_for_song_variations(
    store: &StoreData,
    song: &mut Song,
) -> BotResult<Option<Track>> {
    static COMMON_AFFIXES: &str =
        r"feat\.(\s\w+)|official(\svideo)?|remastered|revisited|(with\s)?lyrics";
    lazy_static::lazy_static! {
        static ref COMMON_ADDITIONS: Regex = Regex::new(format!(r"(?i)\[.*\]|#\w+|\(?[^\w\s]*\s?({})[^\w\s]*\s?\)?", COMMON_AFFIXES).as_str()).unwrap();
    }
    let mut query = COMMON_ADDITIONS.replace_all(song.title(), " ").to_string();
    query = query.replace(|c| c != ' ' && !char::is_alphanumeric(c), "");

    log::debug!("Searching for youtube song");
    if let Some(track) = store.spotify_api.search_for_song(&query).await? {
        let similarity = trigram::similarity(
            &format!(
                "{} {}",
                artists_to_string(track.artists.clone()),
                track.name
            ),
            &query,
        );
        if similarity > 0.3 {
            log::debug!("Result is similar enough ({}). Returning track", similarity);
            return Ok(Some(track));
        }
        log::debug!("Result is not similar enough");
    }
    log::debug!("No result found");

    Ok(None)
}

/// Creates a string from a vector of artists
pub fn artists_to_string(artists: Vec<ArtistSimplified>) -> String {
    artists
        .into_iter()
        .map(|a| a.name)
        .collect::<Vec<String>>()
        .join("&")
}
