use std::{fmt::Debug, marker::PhantomData};

use reqwest::Method;
use serde::Serialize;
use serde_json::{json, Value};

use crate::{
    auth::{AuthFlow, Authorised},
    client::Body,
    error::Result,
    model::{
        player::{CurrentlyPlayingItem, Device, Devices, PlayHistory, PlaybackState, Queue},
        CursorPage,
    },
    Nil,
};

use super::{Client, Endpoint};

impl Endpoint for TransferPlaybackEndpoint {}
impl Endpoint for StartPlaybackEndpoint {}
impl Endpoint for SeekToPositionEndpoint {}
impl Endpoint for SetRepeatModeEndpoint {}
impl Endpoint for SetPlaybackVolumeEndpoint {}
impl Endpoint for ToggleShuffleEndpoint {}
impl<T: TimestampMarker> Endpoint for RecentlyPlayedTracksEndpoint<T> {
    fn endpoint_url(&self) -> &'static str {
        "/me/player/recently-played"
    }
}
impl Endpoint for AddItemToQueueEndpoint {}

// authorised only
pub async fn get_playback_state(
    market: Option<&str>,
    spotify: &Client<impl AuthFlow + Authorised>,
) -> Result<PlaybackState> {
    let market = market.map(|m| [("market", m)]);
    spotify
        .get::<[(&str, &str); 1], _>("/me/player".to_owned(), market)
        .await
}

// authorised only
pub fn transfer_playback(device_id: impl Into<String>) -> TransferPlaybackEndpoint {
    TransferPlaybackEndpoint {
        device_ids: vec![device_id.into()],
        play: None,
    }
}

// authorised only
pub async fn get_available_devices(
    spotify: &Client<impl AuthFlow + Authorised>,
) -> Result<Vec<Device>> {
    spotify
        .get::<(), _>("/me/player/devices".to_owned(), None)
        .await
        .map(|d: Devices| d.devices)
}

// authorised only
pub async fn get_currently_playing_track(
    market: Option<&str>,
    spotify: &Client<impl AuthFlow + Authorised>,
) -> Result<CurrentlyPlayingItem> {
    let market = market.map(|m| [("market", m)]);
    spotify
        .get::<Option<[(&str, &str); 1]>, _>("/me/player/currently-playing".to_owned(), market)
        .await
}

// authorised only
pub fn start_playback() -> StartPlaybackEndpoint {
    StartPlaybackEndpoint::default()
}

// authorised only
pub async fn pause_playback(
    device_id: Option<&str>,
    spotify: &Client<impl AuthFlow + Authorised>,
) -> Result<Nil> {
    let device_id = device_id.map(|d| [("device_id", d)]);
    spotify
        .request(Method::PUT, "/me/player/pause".to_owned(), device_id, None)
        .await
}

// authorised only
pub async fn skip_to_next(
    device_id: Option<&str>,
    spotify: &Client<impl AuthFlow + Authorised>,
) -> Result<Nil> {
    let device_id = device_id.map(|d| [("device_id", d)]);
    spotify
        .request(Method::POST, "/me/player/next".to_owned(), device_id, None)
        .await
}

// authorised only
pub async fn skip_to_previous(
    device_id: Option<&str>,
    spotify: &Client<impl AuthFlow + Authorised>,
) -> Result<Nil> {
    let device_id = device_id.map(|d| [("device_id", d)]);
    spotify
        .request(
            Method::POST,
            "/me/player/previous".to_owned(),
            device_id,
            None,
        )
        .await
}

// authorised only
pub fn seek_to_position(position: u32) -> SeekToPositionEndpoint {
    SeekToPositionEndpoint {
        position_ms: position,
        device_id: None,
    }
}

// authorised only
pub fn set_repeat_mode(repeat_mode: RepeatMode) -> SetRepeatModeEndpoint {
    SetRepeatModeEndpoint {
        state: repeat_mode,
        device_id: None,
    }
}

// authorised only
pub fn set_playback_volume(volume: u32) -> SetPlaybackVolumeEndpoint {
    SetPlaybackVolumeEndpoint {
        volume_percent: volume,
        device_id: None,
    }
}

// authorised only
pub fn toggle_playback_shuffle(shuffle: bool) -> ToggleShuffleEndpoint {
    ToggleShuffleEndpoint {
        state: shuffle,
        device_id: None,
    }
}

// authorised only
pub fn recently_played_tracks() -> RecentlyPlayedTracksEndpoint {
    RecentlyPlayedTracksEndpoint::default()
}

// authorised only
pub async fn get_user_queue(spotify: &Client<impl AuthFlow + Authorised>) -> Result<Queue> {
    spotify
        .get::<(), _>("/me/player/queue".to_owned(), None)
        .await
}

// authorised only
pub fn add_item_to_queue(uri: impl Into<String>) -> AddItemToQueueEndpoint {
    AddItemToQueueEndpoint {
        uri: uri.into(),
        device_id: None,
    }
}

#[derive(Clone, Copy, Debug, Default, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RepeatMode {
    Track,
    Context,
    #[default]
    Off,
}

mod private {
    use super::{After, Before, Unspecified};

    pub trait Sealed {}

    impl Sealed for After {}
    impl Sealed for Before {}
    impl Sealed for Unspecified {}
}

pub trait TimestampMarker: private::Sealed + Debug {}
impl TimestampMarker for Before {}
impl TimestampMarker for After {}
impl TimestampMarker for Unspecified {}

#[derive(Clone, Copy, Debug, Default)]
pub struct After;

#[derive(Clone, Copy, Debug, Default)]
pub struct Before;

#[derive(Clone, Copy, Debug, Default)]
pub struct Unspecified;

#[derive(Clone, Debug, Default, Serialize)]
pub struct TransferPlaybackEndpoint {
    pub(crate) device_ids: Vec<String>,
    pub(crate) play: Option<bool>,
}

impl TransferPlaybackEndpoint {
    /// If `true`, ensure playback happens on the new device.
    /// Otherwise, keep the current playback state.
    pub fn play(mut self, play: bool) -> Self {
        self.play = Some(play);
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn send(self, spotify: &Client<impl AuthFlow + Authorised>) -> Result<Nil> {
        spotify.put("/me/player".to_owned(), Body::Json(self)).await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct StartPlaybackEndpoint {
    #[serde(skip)]
    pub(crate) device_id: Option<String>,
    pub(crate) context_uri: Option<String>,
    pub(crate) uris: Option<Vec<String>>,
    pub(crate) offset: Option<Value>,
    pub(crate) position_ms: Option<u32>,
}

impl StartPlaybackEndpoint {
    #[doc = include_str!("../docs/device_id.md")]
    pub fn device_id(mut self, device_id: impl Into<String>) -> Self {
        self.device_id = Some(device_id.into());
        self
    }

    /// The *URI* of the context to play. Valid contexts are albums, artists and playlists.
    pub fn context_uri(mut self, context_uri: impl Into<String>) -> Self {
        self.context_uri = Some(context_uri.into());
        self
    }

    /// The *URI*s of the tracks to play.
    pub fn uris(mut self, uris: &[&str]) -> Self {
        self.uris = Some(uris.iter().map(ToString::to_string).collect());
        self
    }

    #[doc = include_str!("../docs/offset.md")]
    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(json!({ "position": offset }));
        self
    }

    /// The position at which to start/resume the playback.
    pub fn position_ms(mut self, position_ms: u32) -> Self {
        self.position_ms = Some(position_ms);
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn send(self, spotify: &Client<impl AuthFlow + Authorised>) -> Result<Nil> {
        let endpoint = match self.device_id {
            Some(ref id) => format!("/me/player/play?device_id={id}"),
            None => "/me/player/play".to_owned(),
        };

        spotify.put(endpoint, Body::Json(self)).await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct SeekToPositionEndpoint {
    pub(crate) position_ms: u32,
    pub(crate) device_id: Option<String>,
}

impl SeekToPositionEndpoint {
    #[doc = include_str!("../docs/device_id.md")]
    pub fn device_id(mut self, device_id: impl Into<String>) -> Self {
        self.device_id = Some(device_id.into());
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn send(self, spotify: &Client<impl AuthFlow + Authorised>) -> Result<Nil> {
        spotify
            .request(Method::PUT, "/me/player/seek".to_owned(), self.into(), None)
            .await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct SetRepeatModeEndpoint {
    pub(crate) state: RepeatMode,
    pub(crate) device_id: Option<String>,
}

impl SetRepeatModeEndpoint {
    #[doc = include_str!("../docs/device_id.md")]
    pub fn device_id(mut self, device_id: impl Into<String>) -> Self {
        self.device_id = Some(device_id.into());
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn send(self, spotify: &Client<impl AuthFlow + Authorised>) -> Result<Nil> {
        spotify
            .request(
                Method::PUT,
                "/me/player/repeat".to_owned(),
                self.into(),
                None,
            )
            .await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct SetPlaybackVolumeEndpoint {
    pub(crate) volume_percent: u32,
    pub(crate) device_id: Option<String>,
}

impl SetPlaybackVolumeEndpoint {
    #[doc = include_str!("../docs/device_id.md")]
    pub fn device_id(mut self, device_id: impl Into<String>) -> Self {
        self.device_id = Some(device_id.into());
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn send(self, spotify: &Client<impl AuthFlow + Authorised>) -> Result<Nil> {
        spotify
            .request(
                Method::PUT,
                "/me/player/volume".to_owned(),
                self.into(),
                None,
            )
            .await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct ToggleShuffleEndpoint {
    pub(crate) state: bool,
    pub(crate) device_id: Option<String>,
}

impl ToggleShuffleEndpoint {
    #[doc = include_str!("../docs/device_id.md")]
    pub fn device_id(mut self, device_id: impl Into<String>) -> Self {
        self.device_id = Some(device_id.into());
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn send(self, spotify: &Client<impl AuthFlow + Authorised>) -> Result<Nil> {
        spotify
            .request(
                Method::PUT,
                "/me/player/shuffle".to_owned(),
                self.into(),
                None,
            )
            .await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct RecentlyPlayedTracksEndpoint<T: TimestampMarker = Unspecified> {
    pub(crate) limit: Option<u32>,
    pub(crate) after: Option<u64>,
    pub(crate) before: Option<u64>,
    marker: PhantomData<T>,
}

impl RecentlyPlayedTracksEndpoint<Unspecified> {
    /// A Unix timestamp in miliseconds. Returns all items after (but not including) this cursor position.
    pub fn after(self, after: u64) -> RecentlyPlayedTracksEndpoint<After> {
        RecentlyPlayedTracksEndpoint {
            limit: self.limit,
            after: Some(after),
            before: self.before,
            marker: PhantomData,
        }
    }

    /// A Unix timestamp in miliseconds. Returns all items before (but not including) this cursor position.
    pub fn before(self, before: u64) -> RecentlyPlayedTracksEndpoint<Before> {
        RecentlyPlayedTracksEndpoint {
            limit: self.limit,
            after: self.after,
            before: Some(before),
            marker: PhantomData,
        }
    }
}

impl<T: TimestampMarker + Default> RecentlyPlayedTracksEndpoint<T> {
    #[doc = include_str!("../docs/limit.md")]
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn get(
        self,
        spotify: &Client<impl AuthFlow + Authorised>,
    ) -> Result<CursorPage<PlayHistory, Self>> {
        spotify
            .get("/me/player/recently-played".to_owned(), self)
            .await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct AddItemToQueueEndpoint {
    pub(crate) uri: String,
    pub(crate) device_id: Option<String>,
}

impl AddItemToQueueEndpoint {
    #[doc = include_str!("../docs/device_id.md")]
    pub fn device_id(mut self, device_id: impl Into<String>) -> Self {
        self.device_id = Some(device_id.into());
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn send(self, spotify: &Client<impl AuthFlow + Authorised>) -> Result<Nil> {
        spotify
            .request(
                Method::POST,
                "/me/player/queue".to_owned(),
                self.into(),
                None,
            )
            .await
    }
}
