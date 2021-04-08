use crate::utils::error::{BotError, BotResult};
use aspotify::{Client, ClientCredentials, PlaylistItem, PlaylistItemType};

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

        Self { client }
    }

    /// Returns the song names for a playlist
    pub async fn get_songs_in_playlist(&self, url: &str) -> BotResult<Vec<String>> {
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

        let song_names = playlist_tracks
            .into_iter()
            .filter_map(|item| item.item)
            .map(|t| match t {
                PlaylistItemType::Track(t) => format!(
                    "{} - {}",
                    t.artists
                        .into_iter()
                        .map(|a| a.name)
                        .collect::<Vec<String>>()
                        .join(" & "),
                    t.name
                ),
                PlaylistItemType::Episode(e) => e.name,
            })
            .collect();

        Ok(song_names)
    }

    /// Returns the tracks of a playlist with pagination
    async fn get_tracks_in_playlist(
        &self,
        id: &str,
        limit: usize,
        offset: usize,
    ) -> BotResult<Vec<PlaylistItem>> {
        let tracks = self
            .client
            .playlists()
            .get_playlists_items(id, limit, offset, None)
            .await?
            .data;

        Ok(tracks.items)
    }

    /// Returns all song names for a given album
    pub async fn get_songs_in_album(&self, url: &str) -> BotResult<Vec<String>> {
        let id = self.get_id_for_url(url)?;
        let album = self.client.albums().get_album(&*id, None).await?.data;
        let song_names = album
            .tracks
            .items
            .into_iter()
            .map(|item| {
                format!(
                    "{} - {}",
                    item.artists
                        .into_iter()
                        .map(|a| a.name)
                        .collect::<Vec<String>>()
                        .join(" & "),
                    item.name
                )
            })
            .collect();

        Ok(song_names)
    }

    /// Returns the name for a spotify song url
    pub async fn get_song_name(&self, url: &str) -> BotResult<String> {
        let id = self.get_id_for_url(url)?;
        let track = self.client.tracks().get_track(&*id, None).await?.data;

        Ok(format!(
            "{} - {}",
            track
                .artists
                .into_iter()
                .map(|a| a.name)
                .collect::<Vec<String>>()
                .join(" & "),
            track.name
        ))
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
