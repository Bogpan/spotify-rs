use serde::Serialize;
use serde_json::json;

use crate::{
    auth::AuthFlow,
    client::Body,
    model::{
        track::{SavedTrack, Track, Tracks},
        Page,
    },
    query_list, Nil, Result,
};

use super::{Builder, Endpoint, Limit};

impl Endpoint for TrackEndpoint {}
impl Endpoint for TracksEndpoint {}
impl Endpoint for SavedTracksEndpoint {}

#[derive(Clone, Debug, Default, Serialize)]
pub struct TrackEndpoint {
    #[serde(skip)]
    pub(crate) id: String,
    pub(crate) market: Option<String>,
}

impl<F: AuthFlow> Builder<'_, F, TrackEndpoint> {
    pub fn market(mut self, market: &str) -> Self {
        self.endpoint.market = Some(market.to_owned());
        self
    }

    pub async fn get(self) -> Result<Track> {
        self.spotify
            .get(format!("/tracks/{}", self.endpoint.id), self.endpoint)
            .await
    }
}
#[derive(Clone, Debug, Default, Serialize)]
pub struct TracksEndpoint {
    pub(crate) ids: String,
    pub(crate) market: Option<String>,
}

impl<F: AuthFlow> Builder<'_, F, TracksEndpoint> {
    pub fn market(mut self, market: &str) -> Self {
        self.endpoint.market = Some(market.to_owned());
        self
    }

    pub async fn get(self) -> Result<Vec<Track>> {
        self.spotify
            .get("/tracks".to_owned(), self.endpoint)
            .await
            .map(|t: Tracks| t.tracks)
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct SavedTracksEndpoint {
    pub(crate) market: Option<String>,
    pub(crate) limit: Option<Limit>,
    pub(crate) offset: Option<u32>,
}

impl<F: AuthFlow> Builder<'_, F, SavedTracksEndpoint> {
    pub fn market(mut self, market: &str) -> Self {
        self.endpoint.market = Some(market.to_owned());
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.endpoint.limit = Some(Limit::new(limit));
        self
    }

    pub fn offset(mut self, offset: u32) -> Self {
        self.endpoint.offset = Some(offset);
        self
    }

    pub async fn get(self) -> Result<Page<SavedTrack>> {
        self.spotify
            .get("/me/tracks".to_owned(), self.endpoint)
            .await
    }

    pub async fn save<T: Serialize>(self, ids: &[T]) -> Result<Nil> {
        self.spotify
            .put("/me/tracks".to_owned(), Body::Json(json!({ "ids": ids })))
            .await
    }

    pub async fn remove<T: Serialize>(self, ids: &[T]) -> Result<Nil> {
        self.spotify
            .delete("/me/tracks".to_owned(), Body::Json(json!({ "ids": ids })))
            .await
    }

    pub async fn check<T: AsRef<str>>(self, ids: &[T]) -> Result<Vec<bool>> {
        self.spotify
            .get("/me/tracks/contains".to_owned(), [("ids", query_list(ids))])
            .await
    }
}
