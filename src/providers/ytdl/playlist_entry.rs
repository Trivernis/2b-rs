use serde_derive::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub(crate) struct PlaylistEntry {
    ie_key: String,
    id: String,
    pub url: String,
    pub title: String,
}
