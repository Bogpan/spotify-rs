use chrono::{DateTime, Utc};
use serde::Deserialize;
use spotify_rs_macros::docs;

use super::{track::Track, *};

/// The current user's playback state.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct PlaybackState {
    /// The currently active device.
    pub device: Option<Device>,
    /// The repeat state.
    pub repeat_state: Option<RepeatState>,
    /// Whether or not shuffle is enabled.
    pub shuffle_state: Option<bool>,
    /// The context the item is being played from. (e.g. arist, playlist, album
    /// or show.
    pub context: Option<Context>,
    /// A Unix timestamp of when the playback state was last changed.
    pub timestamp: u64,
    /// The playback position in miliseconds.
    pub progress_ms: Option<u32>,
    /// Whether or not the user is playing something.
    pub is_playing: bool,
    /// The currently playing item - a track or episode.
    pub item: Option<PlayableItem>,
    /// The type of the currently playing item. It may have more variants than `item`.
    pub currently_playing_type: CurrentlyPlayingType,
    /// Allows to update the user interface based on which playback actions
    /// are currently available.
    pub actions: Actions,
}

/// A device.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Device {
    /// The device ID. It is unique and may be persistent, but persistence is
    /// not guaranteed, so it shouldn't be cached for long periods.
    pub id: Option<String>,
    /// Whether or not the device is currently active.
    pub is_active: bool,
    /// Whether or not the device is in a private playback session.
    pub is_private_session: bool,
    /// Whether or not the device is currently restricted. If `true`, the Web API  commands won't be accepted by the device.
    pub is_restricted: bool,
    /// The human-readable name for the device.
    pub name: String,
    /// The type of the device (e.g. computer, smartphone, speaker).
    pub r#type: String,
    /// The current volume percentage.
    pub volume_percent: Option<u32>,
    /// Whether or not the device allows setting the volume.
    pub supports_volume: bool,
}

// Used only to deserialize JSON responses with arrays that are named objects.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub(crate) struct Devices {
    pub(crate) devices: Vec<Device>,
}

/// The context an item is played from.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[docs]
pub struct Context {
    /// The type of the context (e.g. artist, playlist, album, show).
    pub r#type: String,
    pub href: String,
    pub external_urls: ExternalUrls,
    pub uri: String,
}

/// Allows to update the user interface based on which playback actions
/// are currently available.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Actions {
    /// The disallowed actions.
    pub disallows: Disallows,
}

/// Contains (optional) disallowewd actions.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Disallows {
    pub interrupting_playback: Option<bool>,
    pub pausing: Option<bool>,
    pub resuming: Option<bool>,
    pub seeking: Option<bool>,
    pub skipping_next: Option<bool>,
    pub skipping_prev: Option<bool>,
    pub toggling_repeat_context: Option<bool>,
    pub toggling_shuffle: Option<bool>,
    pub toggling_repeat_track: Option<bool>,
    pub transferring_playback: Option<bool>,
}

/// Represents the history entry of a played item.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct PlayHistory {
    /// The track that was played.
    pub track: Track,
    /// The date and time the track was played.
    pub played_at: DateTime<Utc>,
    /// The context the track was played from.
    pub context: Option<Context>,
}

/// A user's queue.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Queue {
    /// The currently playing item.
    pub currently_playing: Option<PlayableItem>,
    /// The items in the queue.
    pub queue: Vec<PlayableItem>,
}

/// Represents the item that's currently playing.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct CurrentlyPlayingItem {
    /// The context the track is being played from.
    pub context: Option<Context>,
    /// A Unix timestamp of when the playback state was last changed.
    pub timestamp: u64,
    /// The playback position in miliseconds.
    pub progress_ms: Option<u32>,
    /// Whether or not the track is currently playing.
    pub is_playing: bool,
    /// The currently playing item - a track or episode.
    pub item: Option<PlayableItem>,
    /// The type of the currently playing item. It may have more variants than `item`.
    pub currently_playing_type: CurrentlyPlayingType,
    /// Allows to update the user interface based on which playback actions
    /// are currently available.
    pub actions: Actions,
}

/// The repeat state of the playback.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RepeatState {
    /// After the current item ends, it won't repeat.
    Off,
    /// After the current item ends, the item will be repeated.
    Track,
    /// After the current item ends, the context of the item will be repeated
    /// (e.g. the playlist).
    Context,
}

/// The type of the currently playing item.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CurrentlyPlayingType {
    /// A track.
    Track,
    /// An episode.
    Episode,
    /// An ad.
    Ad,
    /// An unknown item.
    Unknown,
}
