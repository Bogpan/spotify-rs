use chrono::{DateTime, Utc};
use serde::Deserialize;
use spotify_rs_macros::docs;

use super::{user::ReferenceUser, *};

/// A playlist.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[docs]
pub struct Playlist {
    /// Whether or not other users besides the owner are allowed to modify the playlist.
    pub collaborative: bool,
    /// The playlist's description.
    ///
    /// Note: it's only returned for modified, verified playlists.
    pub description: Option<String>,
    pub external_urls: ExternalUrls,
    /// The followers of the playlist.
    pub followers: Followers,
    pub href: String,
    pub id: String,
    #[serde(deserialize_with = "null_to_default")]
    pub images: Vec<Image>,
    pub name: String,
    /// The owner of the playlist.
    pub owner: ReferenceUser,
    /// Whether or not the playlist is public (if it's added to the user's profile).
    pub public: Option<bool>,
    /// The ID for the current version of the playlist. It can be used in
    /// requests to target a specific playlist version.
    pub snapshot_id: String,
    /// The playlist's tracks.
    pub tracks: Page<PlaylistItem>,
    pub r#type: String,
    pub uri: String,
}

/// A simplified playlist, missing some details, that is usually obtained
/// through endpoints not specific to playlists. The `href` field may be
/// used to get a full playlist.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[docs(name = "playlist")]
pub struct SimplifiedPlaylist {
    /// Whether or not other users besides the owner are allowed to modify the playlist.
    pub collaborative: bool,
    /// The playlist's description.
    ///
    /// Note: it's only returned for modified, verified playlists.
    pub description: Option<String>,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    #[serde(deserialize_with = "null_to_default")]
    pub images: Vec<Image>,
    pub name: String,
    /// The owner of the playlist.
    pub owner: ReferenceUser,
    /// Whether or not the playlist is public (if it's added to the user's profile).
    pub public: Option<bool>,
    /// The ID for the current version of the playlist. It can be used in
    /// requests to target a specific playlist version.
    pub snapshot_id: String,
    /// The playlist's tracks.
    pub tracks: Option<TrackReference>,
    pub r#type: String,
    pub uri: String,
}

// Used only to deserialize JSON responses with arrays that are named objects.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub(crate) struct Playlists {
    pub(crate) playlists: Page<SimplifiedPlaylist>,
}

/// A track or episode within a playlist.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct PlaylistItem {
    /// The date and time the item was added.
    ///
    /// Note: some very old playlists may return `None` in this field.
    pub added_at: Option<DateTime<Utc>>,
    /// The user who added the track or episode.
    ///
    /// Note: some very old playlists may return `None` in this field.
    pub added_by: Option<ReferenceUser>,
    /// Whether or not this item is a local file.
    pub is_local: bool,
    /// The item itself.
    pub track: PlayableItem,
}

/// A list of featured playlists.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct FeaturedPlaylists {
    /// The message of the playlist.
    pub message: String,
    /// The playlists.
    pub playlists: Page<SimplifiedPlaylist>,
}

/// Contains the link where the full details of a playlist's tracks can be found,
/// as well as the number of the tracks in the playlist.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct TrackReference {
    /// A link to the Spotify Web API endpoint providing full details of the
    /// playlist's tracks.
    pub href: String,
    /// The number of tracks in the playlist.
    pub total: u32,
}

// Used only to deserialize JSON responses that are named objects.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub(crate) struct SnapshotId {
    pub(crate) snapshot_id: String,
}
