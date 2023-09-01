use std::marker::PhantomData;

use reqwest::Method;
use serde::Serialize;
use serde_json::{json, Value};

use crate::{
    auth::{AuthFlow, Verifier},
    client::Body,
    error::Result,
    model::{player::PlayHistory, CursorPage},
    Nil,
};

use super::{Builder, Endpoint};

impl Endpoint for TransferPlaybackEndpoint {}
impl Endpoint for StartPlaybackEndpoint {}
impl Endpoint for SeekToPositionEndpoint {}
impl Endpoint for SetRepeatModeEndpoint {}
impl Endpoint for SetPlaybackVolumeEndpoint {}
impl Endpoint for ToggleShuffleEndpoint {}
impl<T: TimestampMarker> Endpoint for RecentlyPlayedTracksEndpoint<T> {}
impl Endpoint for AddItemToQueueEndpoint {}

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

pub trait TimestampMarker: private::Sealed {}
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

impl<F: AuthFlow, V: Verifier> Builder<'_, F, V, TransferPlaybackEndpoint> {
    /// If `true`, ensure playback happens on the new device.
    /// Otherwise, keep the current playback state.
    pub fn play(mut self, play: bool) -> Self {
        self.endpoint.play = Some(play);
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn send(self) -> Result<Nil> {
        self.spotify
            .put("/me/player".to_owned(), Body::Json(self.endpoint))
            .await
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

impl<F: AuthFlow, V: Verifier> Builder<'_, F, V, StartPlaybackEndpoint> {
    #[doc = include_str!("../docs/device_id.md")]
    pub fn device_id(mut self, device_id: &str) -> Self {
        self.endpoint.device_id = Some(format!("?device_id={device_id}"));
        self
    }

    /// The *URI* of the context to play. Valid contexts are albums, artists and playlists.
    pub fn context_uri(mut self, context_uri: &str) -> Self {
        self.endpoint.context_uri = Some(context_uri.to_owned());
        self
    }

    /// The *URI*s of the tracks to play.
    pub fn uris(mut self, uris: &[&str]) -> Self {
        self.endpoint.uris = Some(uris.iter().map(ToString::to_string).collect());
        self
    }

    #[doc = include_str!("../docs/offset.md")]
    pub fn offset(mut self, offset: u32) -> Self {
        self.endpoint.offset = Some(json!({ "position": offset }));
        self
    }

    /// The position at which to start/resume the playback.
    pub fn position_ms(mut self, position_ms: u32) -> Self {
        self.endpoint.position_ms = Some(position_ms);
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn send(self) -> Result<Nil> {
        let device_id = self.endpoint.device_id.as_deref().unwrap_or("");

        self.spotify
            .put(
                format!("/me/player/play{}", device_id),
                Body::Json(self.endpoint),
            )
            .await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct SeekToPositionEndpoint {
    pub(crate) position_ms: u32,
    pub(crate) device_id: Option<String>,
}

impl<F: AuthFlow, V: Verifier> Builder<'_, F, V, SeekToPositionEndpoint> {
    #[doc = include_str!("../docs/device_id.md")]
    pub fn device_id(mut self, device_id: &str) -> Self {
        self.endpoint.device_id = Some(device_id.to_owned());
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn send(self) -> Result<Nil> {
        self.spotify
            .request(
                Method::PUT,
                "/me/player/seek".to_owned(),
                self.endpoint.into(),
                None,
            )
            .await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct SetRepeatModeEndpoint {
    pub(crate) state: RepeatMode,
    pub(crate) device_id: Option<String>,
}

impl<F: AuthFlow, V: Verifier> Builder<'_, F, V, SetRepeatModeEndpoint> {
    #[doc = include_str!("../docs/device_id.md")]
    pub fn device_id(mut self, device_id: &str) -> Self {
        self.endpoint.device_id = Some(device_id.to_owned());
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn send(self) -> Result<Nil> {
        self.spotify
            .request(
                Method::PUT,
                "/me/player/repeat".to_owned(),
                self.endpoint.into(),
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

impl<F: AuthFlow, V: Verifier> Builder<'_, F, V, SetPlaybackVolumeEndpoint> {
    #[doc = include_str!("../docs/device_id.md")]
    pub fn device_id(mut self, device_id: &str) -> Self {
        self.endpoint.device_id = Some(device_id.to_owned());
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn send(self) -> Result<Nil> {
        self.spotify
            .request(
                Method::PUT,
                "/me/player/volume".to_owned(),
                self.endpoint.into(),
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

impl<F: AuthFlow, V: Verifier> Builder<'_, F, V, ToggleShuffleEndpoint> {
    #[doc = include_str!("../docs/device_id.md")]
    pub fn device_id(mut self, device_id: &str) -> Self {
        self.endpoint.device_id = Some(device_id.to_owned());
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn send(self) -> Result<Nil> {
        self.spotify
            .request(
                Method::PUT,
                "/me/player/shuffle".to_owned(),
                self.endpoint.into(),
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

impl<'a, F: AuthFlow, V: Verifier> Builder<'a, F, V, RecentlyPlayedTracksEndpoint<Unspecified>> {
    /// A Unix timestamp in miliseconds. Returns all items after (but not including) this cursor position.
    pub fn after(self, after: u64) -> Builder<'a, F, V, RecentlyPlayedTracksEndpoint<After>> {
        Builder {
            spotify: self.spotify,
            endpoint: RecentlyPlayedTracksEndpoint {
                limit: self.endpoint.limit,
                after: Some(after),
                before: self.endpoint.before,
                marker: PhantomData,
            },
        }
    }

    /// A Unix timestamp in miliseconds. Returns all items before (but not including) this cursor position.
    pub fn before(self, before: u64) -> Builder<'a, F, V, RecentlyPlayedTracksEndpoint<Before>> {
        Builder {
            spotify: self.spotify,
            endpoint: RecentlyPlayedTracksEndpoint {
                limit: self.endpoint.limit,
                after: self.endpoint.after,
                before: Some(before),
                marker: PhantomData,
            },
        }
    }
}

impl<F: AuthFlow, V: Verifier, T: TimestampMarker>
    Builder<'_, F, V, RecentlyPlayedTracksEndpoint<T>>
{
    #[doc = include_str!("../docs/limit.md")]
    pub fn limit(mut self, limit: u32) -> Self {
        self.endpoint.limit = Some(limit);
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn get(self) -> Result<CursorPage<PlayHistory>> {
        self.spotify
            .get("/me/player/recently-played".to_owned(), self.endpoint)
            .await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct AddItemToQueueEndpoint {
    pub(crate) uri: String,
    pub(crate) device_id: Option<String>,
}

impl<F: AuthFlow, V: Verifier> Builder<'_, F, V, AddItemToQueueEndpoint> {
    #[doc = include_str!("../docs/device_id.md")]
    pub fn device_id(mut self, device_id: &str) -> Self {
        self.endpoint.device_id = Some(device_id.to_owned());
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn send(self) -> Result<Nil> {
        self.spotify
            .request(
                Method::POST,
                "/me/player/queue".to_owned(),
                self.endpoint.into(),
                None,
            )
            .await
    }
}
