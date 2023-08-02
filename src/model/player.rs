use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{track::Track, *};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlaybackState {
    pub device: Option<Device>,
    pub repeat_state: Option<RepeatState>,
    pub shuffle_state: Option<bool>,
    pub context: Option<Context>,
    pub timestamp: u64,
    pub progress_ms: Option<u32>,
    pub is_playing: bool,
    pub item: Option<PlayableItem>,
    pub currently_playing_type: CurrentlyPlayingType,
    pub actions: Actions,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Device {
    pub id: Option<String>,
    pub is_active: bool,
    pub is_private_session: bool,
    pub is_restricted: bool,
    pub name: String,
    pub r#type: String,
    pub volume_percent: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Devices {
    pub devices: Vec<Device>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]

pub struct Context {
    pub r#type: String,
    pub href: String,
    pub external_urls: ExternalUrls,
    pub uri: String,
}

/// Allows to update the user interface based on which playback actions are available within the current context.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Actions {
    pub disallows: Disallows,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayHistory {
    pub track: Track,
    pub played_at: DateTime<Utc>,
    pub context: Option<Context>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Queue {
    pub currently_playing: Option<PlayableItem>,
    pub queue: Vec<PlayableItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CurrentlyPlayingTrack {
    pub context: Option<Context>,
    pub timestamp: u64,
    pub progress_ms: Option<u32>,
    pub is_playing: bool,
    pub item: Option<PlayableItem>,
    pub currently_playing_type: CurrentlyPlayingType,
    pub actions: Actions,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepeatState {
    Off,
    Track,
    Context,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CurrentlyPlayingType {
    Track,
    Episode,
    Ad,
    Unknown,
}
