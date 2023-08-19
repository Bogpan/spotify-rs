use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::{album::SimplifiedAlbum, artist::SimplifiedArtist, *};

#[derive(Clone, Debug, Deserialize)]
pub struct Track {
    pub album: SimplifiedAlbum,
    pub artists: Vec<SimplifiedArtist>,
    pub available_markets: Option<Vec<String>>,
    pub disc_number: u32,
    pub duration_ms: u32,
    pub explicit: bool,
    pub external_ids: ExternalIds,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub is_playable: Option<bool>,
    pub linked_from: Option<LinkedFrom>,
    pub restrictions: Option<Restrictions>,
    pub name: String,
    pub popularity: u32,
    pub preview_url: Option<String>,
    pub track_number: u32,
    pub r#type: String,
    pub uri: String,
    pub is_local: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Tracks {
    pub(crate) tracks: Vec<Track>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SimplifiedTrack {
    pub artists: Vec<SimplifiedArtist>,
    #[serde(default)]
    pub available_markets: Vec<String>,
    pub disc_number: u32,
    pub duration_ms: u32,
    pub explicit: bool,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub is_playable: Option<bool>,
    pub linked_from: Option<LinkedFrom>,
    pub restrictions: Option<Restrictions>,
    pub name: String,
    pub preview_url: Option<String>,
    pub track_number: u32,
    pub r#type: String,
    pub uri: String,
    pub is_local: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SavedTrack {
    pub added_at: DateTime<Utc>,
    pub track: Vec<Track>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct LinkedFrom {
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub r#type: String,
    pub uri: String,
}
