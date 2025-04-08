use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{artist::SimplifiedArtist, track::SimplifiedTrack, *};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Album {
    pub album_type: AlbumType,
    pub total_tracks: u32,
    #[serde(default)]
    pub available_markets: Vec<String>,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub release_date: String,
    pub release_date_precision: DatePrecision,
    pub r#type: String,
    pub uri: String,
    pub copyrights: Vec<Copyright>,
    pub external_ids: ExternalIds,
    pub genres: Vec<String>,
    pub label: String,
    pub popularity: u32,
    pub artists: Vec<SimplifiedArtist>,
    pub tracks: Page<SimplifiedTrack>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SimplifiedAlbum {
    pub album_type: AlbumType,
    pub total_tracks: u32,
    #[serde(default)]
    pub available_markets: Vec<String>,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub release_date: String,
    pub release_date_precision: DatePrecision,
    pub restrictions: Option<Restrictions>,
    pub r#type: String,
    pub uri: String,
    /// The field is present when getting an artist's albums. Compared to album_type this field represents the relationship between the artist and the album.
    pub album_group: Option<AlbumGroup>,
    pub artists: Vec<SimplifiedArtist>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SavedAlbum {
    pub added_at: DateTime<Utc>,
    pub album: Album,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct Albums {
    pub(crate) albums: Vec<Album>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct PagedAlbums {
    pub(crate) albums: Page<SimplifiedAlbum>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AlbumType {
    #[serde(alias = "ALBUM")]
    Album,
    #[serde(alias = "SINGLE")]
    Single,
    #[serde(alias = "COMPILATION")]
    Compilation,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AlbumGroup {
    Album,
    Single,
    Compilation,
    AppearsOn,
}

impl AsRef<str> for AlbumGroup {
    fn as_ref(&self) -> &str {
        match self {
            AlbumGroup::Album => "album",
            AlbumGroup::Single => "single",
            AlbumGroup::Compilation => "compilation",
            AlbumGroup::AppearsOn => "appears_on",
        }
    }
}
