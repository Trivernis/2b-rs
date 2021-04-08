use std::collections::VecDeque;

use songbird::tracks::TrackHandle;

use crate::providers::music::responses::{PlaylistEntry, VideoInformation};
use crate::utils::shuffle_vec_deque;

#[derive(Clone, Debug)]
pub struct MusicQueue {
    inner: VecDeque<Song>,
    current: Option<TrackHandle>,
}

impl MusicQueue {
    pub fn new() -> Self {
        Self {
            inner: VecDeque::new(),
            current: None,
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
        shuffle_vec_deque(&mut self.inner)
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
    pub fn set_current(&mut self, handle: TrackHandle) {
        self.current = Some(handle)
    }

    /// Clears the currently playing song
    pub fn clear_current(&mut self) {
        self.current = None;
    }

    /// Returns the reference to the currently playing song
    pub fn current(&self) -> &Option<TrackHandle> {
        &self.current
    }

    /// Clears the queue
    pub fn clear(&mut self) {
        self.inner.clear();
    }
}

#[derive(Clone, Debug)]
pub struct Song {
    pub url: String,
    pub title: String,
    pub author: String,
    pub thumbnail: Option<String>,
}

impl From<VideoInformation> for Song {
    fn from(info: VideoInformation) -> Self {
        Self {
            url: info.webpage_url,
            title: info.title,
            author: info.uploader,
            thumbnail: info.thumbnail,
        }
    }
}

impl From<PlaylistEntry> for Song {
    fn from(entry: PlaylistEntry) -> Self {
        Self {
            url: format!("https://www.youtube.com/watch?v={}", entry.url),
            title: entry.title,
            author: entry.uploader,
            thumbnail: None,
        }
    }
}
