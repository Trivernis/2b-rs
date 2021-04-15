use std::collections::VecDeque;

use aspotify::Track;
use songbird::tracks::TrackHandle;

use bot_coreutils::shuffle::Shuffle;

use crate::providers::music::responses::{PlaylistEntry, VideoInformation};
use crate::providers::music::song_to_youtube_video;
use bot_database::models::YoutubeSong;
use bot_serenityutils::core::MessageHandle;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct MusicQueue {
    inner: VecDeque<Song>,
    current: Option<(TrackHandle, Song)>,
    paused: bool,
    pub now_playing_msg: Option<Arc<RwLock<MessageHandle>>>,
    pub leave_flag: bool,
}

impl MusicQueue {
    pub fn new() -> Self {
        Self {
            inner: VecDeque::new(),
            current: None,
            paused: false,
            leave_flag: false,
            now_playing_msg: None,
        }
    }

    /// Adds a song to the queue
    pub fn add(&mut self, song: Song) {
        self.inner.push_back(song);
    }

    /// Adds a song to be played next in the queue
    pub fn add_next(&mut self, song: Song) {
        self.inner.push_front(song);
    }

    /// Shuffles the queue
    pub fn shuffle(&mut self) {
        self.inner.shuffle()
    }

    /// Returns a reference to the inner deque
    pub fn entries(&self) -> &VecDeque<Song> {
        &self.inner
    }

    /// Returns the next song from the queue
    pub fn next(&mut self) -> Option<Song> {
        self.inner.pop_front()
    }

    /// Sets the currently playing song
    pub fn set_current(&mut self, handle: TrackHandle, song: Song) {
        self.current = Some((handle, song))
    }

    /// Clears the currently playing song
    pub fn clear_current(&mut self) {
        self.current = None;
    }

    /// Returns the reference to the currently playing song
    pub fn current(&self) -> &Option<(TrackHandle, Song)> {
        &self.current
    }

    /// Clears the queue
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /// Moves a song to a new position
    pub fn move_position(&mut self, index: usize, new_index: usize) {
        if let Some(song) = self.inner.remove(index) {
            self.inner.insert(new_index, song);
        }
    }

    /// Removes a song from the queue
    pub fn remove(&mut self, index: usize) {
        self.inner.remove(index);
    }

    /// Toggles pause
    pub fn pause(&mut self) {
        if let Some(current) = &self.current {
            if self.paused {
                let _ = current.0.play();
            } else {
                let _ = current.0.pause();
            }

            self.paused = !self.paused;
        } else {
            self.paused = false;
        }
    }

    /// Returns if the queue is paused
    pub fn paused(&self) -> bool {
        self.paused
    }
}

#[derive(Clone, Debug)]
pub enum SongSource {
    Spotify(Track),
    YouTube(String),
}

#[derive(Clone, Debug)]
pub struct Song {
    url: Option<String>,
    title: String,
    author: String,
    thumbnail: Option<String>,
    source: SongSource,
}

impl Song {
    /// The url of the song
    /// fetched when not available
    pub async fn url(&mut self) -> Option<String> {
        if let Some(url) = self.url.clone() {
            Some(url)
        } else {
            log::debug!("Lazy fetching video for title");
            let information = song_to_youtube_video(&self).await.ok()??;
            self.url = Some(information.webpage_url.clone());
            self.thumbnail = information.thumbnail;
            self.author = information.uploader;

            Some(information.webpage_url)
        }
    }

    /// The title of the song
    pub fn title(&self) -> &String {
        &self.title
    }

    #[allow(dead_code)]
    /// the author of the song
    pub fn author(&self) -> &String {
        &self.author
    }

    /// The thumbnail of the song
    pub fn thumbnail(&self) -> &Option<String> {
        &self.thumbnail
    }

    /// The source of the song
    pub fn source(&self) -> &SongSource {
        &self.source
    }
}

impl From<VideoInformation> for Song {
    fn from(info: VideoInformation) -> Self {
        Self {
            url: Some(info.webpage_url.clone()),
            title: info.title,
            author: info.uploader,
            thumbnail: info.thumbnail,
            source: SongSource::YouTube(info.webpage_url),
        }
    }
}

impl From<PlaylistEntry> for Song {
    fn from(entry: PlaylistEntry) -> Self {
        Self {
            url: Some(format!("https://www.youtube.com/watch?v={}", entry.url)),
            title: entry.title,
            author: entry.uploader,
            thumbnail: None,
            source: SongSource::YouTube(format!("https://www.youtube.com/watch?v={}", entry.url)),
        }
    }
}

impl From<Track> for Song {
    fn from(track: Track) -> Self {
        Self {
            title: track.name.clone(),
            author: track
                .clone()
                .artists
                .into_iter()
                .map(|a| a.name.clone())
                .collect::<Vec<String>>()
                .join(" & "),
            url: None,
            thumbnail: None,
            source: SongSource::Spotify(track),
        }
    }
}

impl From<YoutubeSong> for Song {
    fn from(song: YoutubeSong) -> Self {
        Self {
            title: song.title,
            author: song.artist,
            url: Some(song.url.clone()),
            thumbnail: None,
            source: SongSource::YouTube(song.url),
        }
    }
}
