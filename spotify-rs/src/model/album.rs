use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use spotify_rs_macros::docs;

use super::{artist::SimplifiedArtist, track::SimplifiedTrack, *};

/// An album.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[docs]
pub struct Album {
    /// The type of the album.
    pub album_type: AlbumType,
    /// The number of tracks in the album.
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
    /// A list of the genres the album is associated with. Will be
    /// empty in the case the album hasn't yet been classified.
    pub genres: Vec<String>,
    /// The label associated with the album.
    pub label: String,
    /// A number ranging between `1` - `100` that represents the popularity of
    /// the album.
    pub popularity: u32,
    /// The artists of the album.
    pub artists: Vec<SimplifiedArtist>,
    /// The tracks of the album.
    pub tracks: Page<SimplifiedTrack>,
}

/// A simplified album, missing some details, that is usually obtained
/// through endpoints not specific to albums. The `href` field may be
/// used to get a full album.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[docs(name = "album")]
pub struct SimplifiedAlbum {
    /// The type of the album.
    pub album_type: AlbumType,
    /// The number of tracks in the album.
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
    pub restrictions: Option<Restriction>,
    pub r#type: String,
    pub uri: String,
    /// The field is present when getting an artist's albums. Compared to album_type
    /// this field represents the relationship between the artist and the album.
    pub album_group: Option<AlbumGroup>,
    /// The artists of the album.
    pub artists: Vec<SimplifiedArtist>,
}

/// An album saved by a user.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct SavedAlbum {
    /// The date and time the album was saved.
    pub added_at: DateTime<Utc>,
    /// The album itself.
    pub album: Album,
}

// Used only to deserialize JSON responses with arrays that are named objects.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub(crate) struct Albums {
    pub(crate) albums: Vec<Album>,
}

// Used only to deserialize JSON responses with arrays that are named objects.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub(crate) struct PagedAlbums {
    pub(crate) albums: Page<SimplifiedAlbum>,
}

// The variants only need to be renamde to lowercase, but I'm using snake_case
// for consistency's sake. The aliases are because the docs say the album types
// are lowercase, but they're uppercase too sometimes.
/// The type of an album.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum AlbumType {
    /// An album.
    #[serde(alias = "ALBUM")]
    Album,

    /// A single.
    #[serde(alias = "SINGLE")]
    Single,

    /// A compilation.
    #[serde(alias = "COMPILATION")]
    Compilation,

    /// An EP.
    #[serde(alias = "EP")]
    Ep,

    /// Any other album type that may be added in the future.
    #[serde(untagged)]
    Unknown(String),
}

/// The album group, which describes the relation between the artist and the album.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AlbumGroup {
    /// An album.
    Album,
    /// A single.
    Single,
    /// A compilation.
    Compilation,
    /// An album on which the given artist appears.
    AppearsOn,
}

// Enable easy serialization of `include_groups` when getting an artist's albums,
// by allowing the value to be passed to `query_list()`.
#[doc(hidden)]
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
