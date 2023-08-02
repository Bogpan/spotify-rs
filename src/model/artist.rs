use serde::{Deserialize, Serialize};

use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimplifiedArtist {
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub name: String,
    pub r#type: String,
    pub uri: String,
}
