use serde_derive::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub(crate) struct PlaylistEntry {
    ie_key: String,
    id: String,
    pub url: String,
    pub title: String,
    pub uploader: String,
}

#[derive(Deserialize, Clone, Debug)]
pub(crate) struct VideoInformation {
    id: String,
    pub title: String,
    pub thumbnail: Option<String>,
    pub webpage_url: String,
    pub uploader: String,
}
