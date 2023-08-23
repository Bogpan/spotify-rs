use reqwest::Method;
use serde::Serialize;
use serde_json::{json, Value};

use crate::{
    auth::AuthFlow,
    client::Body,
    model::{player::PlayHistory, CursorPage},
    Nil, Result,
};

use super::{Builder, Endpoint};

impl Endpoint for TransferPlaybackEndpoint {}
impl Endpoint for StartPlaybackEndpoint {}
impl Endpoint for SeekToPositionEndpoint {}
impl Endpoint for SetRepeatModeEndpoint {}
impl Endpoint for SetPlaybackVolumeEndpoint {}
impl Endpoint for ToggleShuffleEndpoint {}
impl Endpoint for RecentlyPlayedTracksEndpoint {}
impl Endpoint for AddItemToQueueEndpoint {}

#[derive(Clone, Copy, Debug, Default, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RepeatMode {
    Track,
    Context,
    #[default]
    Off,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct TransferPlaybackEndpoint {
    pub(crate) device_ids: Vec<String>,
    pub(crate) play: Option<bool>,
}

impl<F: AuthFlow> Builder<'_, F, TransferPlaybackEndpoint> {
    pub fn play(mut self, play: bool) -> Self {
        self.endpoint.play = Some(play);
        self
    }

    pub async fn transfer(self) -> Result<Nil> {
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

impl<F: AuthFlow> Builder<'_, F, StartPlaybackEndpoint> {
    pub fn device_id(mut self, device_id: &str) -> Self {
        self.endpoint.device_id = Some(format!("?device_id={device_id}"));
        self
    }

    pub fn context_uri(mut self, context_uri: &str) -> Self {
        self.endpoint.context_uri = Some(context_uri.to_owned());
        self
    }

    pub fn uris(mut self, uris: &[&str]) -> Self {
        self.endpoint.uris = Some(uris.iter().map(ToString::to_string).collect());
        self
    }

    pub fn offset(mut self, offset: u32) -> Self {
        self.endpoint.offset = Some(json!({ "position": offset }));
        self
    }

    pub fn position_ms(mut self, position_ms: u32) -> Self {
        self.endpoint.position_ms = Some(position_ms);
        self
    }

    pub async fn start(self) -> Result<Nil> {
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

impl<F: AuthFlow> Builder<'_, F, SeekToPositionEndpoint> {
    pub fn device_id(mut self, device_id: &str) -> Self {
        self.endpoint.device_id = Some(device_id.to_owned());
        self
    }

    pub async fn seek(self) -> Result<Nil> {
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

impl<F: AuthFlow> Builder<'_, F, SetRepeatModeEndpoint> {
    pub fn device_id(mut self, device_id: &str) -> Self {
        self.endpoint.device_id = Some(device_id.to_owned());
        self
    }

    pub async fn set(self) -> Result<Nil> {
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

impl<F: AuthFlow> Builder<'_, F, SetPlaybackVolumeEndpoint> {
    pub fn device_id(mut self, device_id: &str) -> Self {
        self.endpoint.device_id = Some(device_id.to_owned());
        self
    }

    pub async fn set(self) -> Result<Nil> {
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

impl<F: AuthFlow> Builder<'_, F, ToggleShuffleEndpoint> {
    pub fn device_id(mut self, device_id: &str) -> Self {
        self.endpoint.device_id = Some(device_id.to_owned());
        self
    }

    pub async fn toggle(self) -> Result<Nil> {
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
pub struct RecentlyPlayedTracksEndpoint {
    pub(crate) limit: Option<u32>,
    pub(crate) after: Option<u64>,
    pub(crate) before: Option<u64>,
}

impl<F: AuthFlow> Builder<'_, F, RecentlyPlayedTracksEndpoint> {
    pub fn limit(mut self, limit: u32) -> Self {
        self.endpoint.limit = Some(limit);
        self
    }

    pub fn after(mut self, after: u64) -> Self {
        self.endpoint.after = Some(after);
        self
    }

    pub fn before(mut self, before: u64) -> Self {
        self.endpoint.before = Some(before);
        self
    }

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

impl<F: AuthFlow> Builder<'_, F, AddItemToQueueEndpoint> {
    pub fn device_id(mut self, device_id: &str) -> Self {
        self.endpoint.device_id = Some(device_id.to_owned());
        self
    }

    pub async fn add(self) -> Result<Nil> {
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
