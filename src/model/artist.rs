use serde::Deserialize;

use super::*;

#[derive(Clone, Debug, Deserialize)]
pub struct Artist {
    pub external_urls: ExternalUrls,
    pub followers: Followers,
    pub genres: Vec<String>,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub popularity: u32,
    pub r#type: String,
    pub uri: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SimplifiedArtist {
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub name: String,
    pub r#type: String,
    pub uri: String,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Artists {
    pub(crate) artists: Vec<Artist>,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct PagedArtists {
    pub(crate) artists: CursorPage<Artist>,
}
