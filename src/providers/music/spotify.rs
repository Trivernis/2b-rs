use aspotify::{Client, ClientCredentials, PlaylistItem, PlaylistItemType};

use crate::providers::music::queue::Song;
use crate::utils::error::{BotError, BotResult};

pub struct SpotifyApi {
    client: Client,
}

impl SpotifyApi {
    /// Creates a new spotify api wrapper with the credentials stored
    /// in the .env files
    pub fn new() -> Self {
        let credentials = ClientCredentials {
            id: dotenv::var("SPOTIFY_CLIENT_ID").expect("Missing Spotify Credentials"),
            secret: dotenv::var("SPOTIFY_CLIENT_SECRET").expect("Missing Spotify Credentials"),
        };
        let client = Client::new(credentials);
        log::info!("Spotify API initialized.");

        Self { client }
    }

    /// Returns the songs for a playlist
    pub async fn get_songs_in_playlist(&self, url: &str) -> BotResult<Vec<Song>> {
        log::debug!("Fetching spotify songs from playlist '{}'", url);
        let id = self.get_id_for_url(url)?;
        let mut playlist_tracks = Vec::new();
        let mut offset = 0;

        loop {
            let mut tracks = self.get_tracks_in_playlist(&*id, 100, offset).await?;
            if tracks.len() == 0 {
                break;
            }
            playlist_tracks.append(&mut tracks);
            offset += 100;
        }
        log::debug!(
            "{} Songs found in spotify playlist '{}'",
            playlist_tracks.len(),
            url
        );

        let songs = playlist_tracks
            .into_iter()
            .filter_map(|item| item.item)
            .filter_map(|t| match t {
                PlaylistItemType::Track(t) => Some(Song::from(t)),
                PlaylistItemType::Episode(_) => None,
            })
            .collect();
        log::trace!("Songs are {:?}", songs);

        Ok(songs)
    }

    /// Returns the tracks of a playlist with pagination
    async fn get_tracks_in_playlist(
        &self,
        id: &str,
        limit: usize,
        offset: usize,
    ) -> BotResult<Vec<PlaylistItem>> {
        log::trace!(
            "Fetching songs from spotify playlist: limit {}, offset {}",
            limit,
            offset
        );
        let tracks = self
            .client
            .playlists()
            .get_playlists_items(id, limit, offset, None)
            .await?
            .data;
        log::trace!("Tracks are {:?}", tracks);

        Ok(tracks.items)
    }

    /// Returns all songs for a given album
    pub async fn get_songs_in_album(&self, url: &str) -> BotResult<Vec<Song>> {
        log::debug!("Fetching songs for spotify album '{}'", url);
        let id = self.get_id_for_url(url)?;
        let album = self.client.albums().get_album(&*id, None).await?.data;
        log::trace!("Album is {:?}", album);
        let song_names: Vec<Song> = album.tracks.items.into_iter().map(Song::from).collect();
        log::debug!("{} songs found in album '{}'", song_names.len(), url);

        Ok(song_names)
    }

    /// Returns song entity for a given spotify url
    pub async fn get_song_name(&self, url: &str) -> BotResult<Song> {
        log::debug!("Getting song for {}", url);
        let id = self.get_id_for_url(url)?;
        let track = self.client.tracks().get_track(&*id, None).await?.data;
        log::trace!("Track info is {:?}", track);

        Ok(track.into())
    }

    /// Returns the id for a given spotify URL
    fn get_id_for_url(&self, url: &str) -> BotResult<String> {
        url.split('/')
            .last()
            .ok_or(BotError::from("Invalid Spotify URL"))
            .and_then(|s| {
                s.split('?')
                    .next()
                    .ok_or(BotError::from("Invalid Spotify URL"))
            })
            .map(|s| s.to_string())
    }
}
