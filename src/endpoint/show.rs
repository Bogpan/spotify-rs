use serde::Serialize;
use serde_json::json;

use crate::{
    auth::AuthFlow,
    client::Body,
    model::{
        show::{Episode, Episodes, SavedEpisode},
        Page,
    },
    query_list, Nil, Result,
};

use super::{Builder, Endpoint, Limit};

impl Endpoint for EpisodeEndpoint {}
impl Endpoint for EpisodesEndpoint {}
impl Endpoint for SavedEpisodesEndpoint {}

#[derive(Clone, Debug, Default, Serialize)]
pub struct EpisodeEndpoint {
    #[serde(skip)]
    pub(crate) id: String,
    pub(crate) market: Option<String>,
}

impl<F: AuthFlow> Builder<'_, F, EpisodeEndpoint> {
    pub fn market(mut self, market: &str) -> Self {
        self.endpoint.market = Some(market.to_owned());
        self
    }

    pub async fn get(self) -> Result<Episode> {
        self.spotify
            .get(format!("/episodes/{}", self.endpoint.id), self.endpoint)
            .await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct EpisodesEndpoint {
    pub(crate) ids: String,
    pub(crate) market: Option<String>,
}

impl<F: AuthFlow> Builder<'_, F, EpisodesEndpoint> {
    pub fn market(mut self, market: &str) -> Self {
        self.endpoint.market = Some(market.to_owned());
        self
    }

    pub async fn get(self) -> Result<Vec<Episode>> {
        self.spotify
            .get("/episodes/".to_owned(), self.endpoint)
            .await
            .map(|e: Episodes| e.episodes)
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct SavedEpisodesEndpoint {
    pub(crate) market: Option<String>,
    pub(crate) limit: Option<Limit>,
    pub(crate) offset: Option<u32>,
}

impl<F: AuthFlow> Builder<'_, F, SavedEpisodesEndpoint> {
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

    pub async fn get(self) -> Result<Page<SavedEpisode>> {
        self.spotify
            .get("/me/episodes".to_owned(), self.endpoint)
            .await
    }

    pub async fn save<T: Serialize>(self, ids: &[T]) -> Result<Nil> {
        self.spotify
            .put(
                "/me/episodes".to_owned(),
                Body::Json(json!( { "ids": ids })),
            )
            .await
    }

    pub async fn remove<T: Serialize>(self, ids: &[T]) -> Result<Nil> {
        self.spotify
            .delete("/me/episodes".to_owned(), Body::Json(json!( {"ids": ids })))
            .await
    }

    pub async fn check<T: AsRef<str>>(self, ids: &[T]) -> Result<Vec<bool>> {
        self.spotify
            .get::<(), _>(
                format!("/me/episodes/contains?ids={}", query_list(ids)),
                None,
            )
            .await
    }
}
