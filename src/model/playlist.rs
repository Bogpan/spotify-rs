use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::{user::ReferenceUser, *};

#[derive(Clone, Debug, Deserialize)]
pub struct Playlist {
    pub collaborative: bool,
    pub description: Option<String>,
    pub external_urls: ExternalUrls,
    pub followers: Followers,
    pub href: String,
    pub id: String,
    pub images: Option<Vec<Image>>,
    pub name: String,
    pub owner: ReferenceUser,
    pub public: Option<bool>,
    pub snapshot_id: String,
    pub tracks: Page<PlaylistTrack>,
    pub r#type: String,
    pub uri: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SimplifiedPlaylist {
    pub collaborative: bool,
    pub description: Option<String>,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub images: Option<Vec<Image>>,
    pub name: String,
    pub owner: ReferenceUser,
    pub public: Option<bool>,
    pub snapshot_id: String,
    /// A collection containing a link (`href`) to the Web API endpoint where full details of the playlist's tracks can be retrieved,
    /// along with the total number of tracks in the playlist. Note, a track object may be `null`. This can happen if a track is no longer available.
    pub tracks: Option<TrackReference>,
    pub r#type: String,
    pub uri: String,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Playlists {
    pub(crate) playlists: Page<SimplifiedPlaylist>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PlaylistTrack {
    /// The date and time the track or episode was added. Note: some very old playlists may return null in this field.
    pub added_at: Option<DateTime<Utc>>,
    /// The Spotify user who added the track or episode. Note: some very old playlists may return null in this field.
    pub added_by: Option<ReferenceUser>,
    pub is_local: bool,
    pub track: PlayableItem,
}

#[derive(Clone, Debug, Deserialize)]
pub struct FeaturedPlaylists {
    pub message: String,
    pub playlists: Page<SimplifiedPlaylist>,
}

/// A collection containing a link (`href`) to the Web API endpoint where full details of the playlist's tracks can be retrieved,
/// along with the total number of tracks in the playlist. Note, a track object may be `null`. This can happen if a track is no longer available.
#[derive(Clone, Debug, Deserialize)]
pub struct TrackReference {
    pub href: String,
    pub total: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct SnapshotId {
    pub(crate) snapshot_id: String,
}
