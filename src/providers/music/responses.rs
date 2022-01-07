use serde_derive::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub(crate) struct PlaylistEntry {
    #[allow(dead_code)]
    ie_key: String,
    #[allow(dead_code)]
    id: String,
    pub url: String,
    pub title: String,
    pub uploader: String,
}

#[derive(Deserialize, Clone, Debug)]
pub(crate) struct VideoInformation {
    #[allow(dead_code)]
    pub(crate) id: String,
    pub title: String,
    pub thumbnail: Option<String>,
    pub webpage_url: String,
    pub uploader: String,
}
