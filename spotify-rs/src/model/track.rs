use chrono::{DateTime, Utc};
use serde::Deserialize;
use spotify_rs_macros::docs;

use super::{album::SimplifiedAlbum, artist::SimplifiedArtist, *};

/// A track.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[docs]
pub struct Track {
    /// The album the track belongs to.
    pub album: SimplifiedAlbum,
    /// The artists who performed on the track.
    pub artists: Vec<SimplifiedArtist>,
    pub available_markets: Option<Vec<String>>,
    /// The disc number, which us usually `1`, unless the album consists of more
    /// than one disk.
    pub disc_number: u32,
    pub duration_ms: u32,
    pub explicit: bool,
    pub external_ids: ExternalIds,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    /// It's part of the response when
    /// [Track Relinking](https://developer.spotify.com/documentation/web-api/concepts/track-relinking)
    /// is applied.
    pub is_playable: Option<bool>,
    /// It's part of the response when
    /// [Track Relinking](https://developer.spotify.com/documentation/web-api/concepts/track-relinking)
    /// has been applied and the requested track has been replaced with a
    /// different one. This field contains information about the originally
    /// requested track.
    pub linked_from: Option<LinkedFrom>,
    pub restrictions: Option<Restriction>,
    pub name: String,
    /// A value ranging between `0` - `100` that represents the popularity
    /// of a track. The popularity is based mostly on the number of times
    /// the track has been played and how recently it's been played.
    ///
    /// Duplicate tracks - the same track from a single and an album are rated
    /// independently.
    ///
    /// Note: the value may lag behind by a few days, as it's not updated in
    /// real time.
    pub popularity: u32,
    /// The URL for a 30 second MP3 preview of the track.
    ///
    /// **Note:** This attribute has been deprecated by Spotify. It continues to work for
    /// applications already using the extended mode in the API.
    ///
    /// You can read more about this [here](https://developer.spotify.com/blog/2024-11-27-changes-to-the-web-api).
    pub preview_url: Option<String>,
    /// The number of the track.
    pub track_number: u32,
    pub r#type: String,
    pub uri: String,
    /// Whether or not the track is from a local file.
    pub is_local: bool,
}

// Used only to deserialize JSON responses with arrays that are named objects.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub(crate) struct Tracks {
    pub(crate) tracks: Vec<Track>,
}

/// A simplified track, missing some details, that is usually obtained
/// through endpoints not specific to tracks. The `href` field may be
/// used to get a full track.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[docs(name = "track")]
pub struct SimplifiedTrack {
    /// The artists who performed on the track.
    pub artists: Vec<SimplifiedArtist>,
    pub available_markets: Option<Vec<String>>,
    /// The disc number, which us usually `1`, unless the album consists of more
    /// than one disk.
    pub disc_number: u32,
    pub duration_ms: u32,
    pub explicit: bool,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub is_playable: Option<bool>,
    /// It's part of the response when
    /// [Track Relinking](https://developer.spotify.com/documentation/web-api/concepts/track-relinking)
    /// has been applied and the requested track has been replaced with a
    /// different one. This field contains information about the originally
    /// requested track.
    pub linked_from: Option<LinkedFrom>,
    pub restrictions: Option<Restriction>,
    pub name: String,
    /// The URL for a 30 second MP3 preview of the track.
    ///
    /// **Note:** This attribute has been deprecated by Spotify. It continues to work for
    /// applications already using the extended mode in the API.
    ///
    /// You can read more about this [here](https://developer.spotify.com/blog/2024-11-27-changes-to-the-web-api).
    pub preview_url: Option<String>,
    /// The number of the track.
    pub track_number: u32,
    pub r#type: String,
    pub uri: String,
    /// Whether or not the track is from a local file.
    pub is_local: bool,
}

/// A track saved by a user.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct SavedTrack {
    /// The date and time the track was saved.
    pub added_at: DateTime<Utc>,
    /// The track itself.
    pub track: Track,
}

/// Information about a track that's been
/// [relinked](https://developer.spotify.com/documentation/web-api/concepts/track-relinking).
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[docs(name = "track")]
pub struct LinkedFrom {
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: Option<String>,
    pub r#type: String,
    pub uri: String,
}
