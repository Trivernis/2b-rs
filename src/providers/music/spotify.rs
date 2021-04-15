use aspotify::{Client, ClientCredentials, ItemType, PlaylistItemType, Track};

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

    /// Searches for a song on spotify
    pub async fn search_for_song(&self, query: &str) -> BotResult<Option<Track>> {
        log::debug!("Searching for song '{}' on spotify", query);
        let types = vec![ItemType::Track];
        let result = self
            .client
            .search()
            .search(query, types, false, 1, 0, None)
            .await?;
        log::trace!("Result is {:?}", result);
        let tracks = result
            .data
            .tracks
            .ok_or(BotError::from("Failed to get search spotify results"))?;

        Ok(tracks.items.into_iter().next())
    }

    /// Returns the songs for a playlist
    pub async fn get_songs_in_playlist(&self, url: &str) -> BotResult<Vec<Track>> {
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

        log::trace!("Songs are {:?}", playlist_tracks);

        Ok(playlist_tracks)
    }

    /// Returns the tracks of a playlist with pagination
    async fn get_tracks_in_playlist(
        &self,
        id: &str,
        limit: usize,
        offset: usize,
    ) -> BotResult<Vec<Track>> {
        log::trace!(
            "Fetching songs from spotify playlist: limit {}, offset {}",
            limit,
            offset
        );
        let page = self
            .client
            .playlists()
            .get_playlists_items(id, limit, offset, None)
            .await?
            .data;

        let tracks: Vec<Track> = page
            .items
            .into_iter()
            .filter_map(|item| item.item)
            .filter_map(|t| match t {
                PlaylistItemType::Track(t) => Some(t),
                PlaylistItemType::Episode(_) => None,
            })
            .collect();
        log::trace!("Tracks are {:?}", tracks);

        Ok(tracks)
    }

    /// Returns all songs for a given album
    pub async fn get_songs_in_album(&self, url: &str) -> BotResult<Vec<Track>> {
        log::debug!("Fetching songs for spotify album '{}'", url);
        let id = self.get_id_for_url(url)?;
        let album = self.client.albums().get_album(&*id, None).await?.data;
        log::trace!("Album is {:?}", album);

        let simple_tracks: Vec<String> = album
            .tracks
            .items
            .into_iter()
            .filter_map(|t| t.id)
            .collect();
        let tracks = self
            .client
            .tracks()
            .get_tracks(simple_tracks, None)
            .await?
            .data;

        log::trace!("Tracks are {:?}", tracks);

        Ok(tracks)
    }

    /// Returns song entity for a given spotify url
    pub async fn get_song_name(&self, url: &str) -> BotResult<Track> {
        log::debug!("Getting song for {}", url);
        let id = self.get_id_for_url(url)?;
        let track = self.client.tracks().get_track(&*id, None).await?.data;
        log::trace!("Track info is {:?}", track);

        Ok(track)
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
