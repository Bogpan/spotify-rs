use serde::Serialize;
use serde_json::json;

use crate::{
    auth::AuthFlow,
    client::Body,
    model::{
        show::{
            Episode, Episodes, SavedEpisode, SavedShow, Show, Shows, SimplifiedEpisode,
            SimplifiedShow,
        },
        Page,
    },
    query_list, Nil, Result,
};

use super::{Builder, Endpoint, Limit};

impl Endpoint for ShowEndpoint {}
impl Endpoint for ShowsEndpoint {}
impl Endpoint for ShowEpisodesEndpoint {}
impl Endpoint for SavedShowsEndpoint {}
impl Endpoint for EpisodeEndpoint {}
impl Endpoint for EpisodesEndpoint {}
impl Endpoint for SavedEpisodesEndpoint {}

#[derive(Clone, Debug, Default, Serialize)]
pub struct ShowEndpoint {
    #[serde(skip)]
    pub(crate) id: String,
    pub(crate) market: Option<String>,
}

impl<F: AuthFlow> Builder<'_, F, ShowEndpoint> {
    pub fn market(mut self, market: &str) -> Self {
        self.endpoint.market = Some(market.to_owned());
        self
    }

    pub async fn get(self) -> Result<Show> {
        self.spotify
            .get(format!("/shows/{}", self.endpoint.id), self.endpoint)
            .await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct ShowsEndpoint {
    pub(crate) ids: String,
    pub(crate) market: Option<String>,
}

impl<F: AuthFlow> Builder<'_, F, ShowsEndpoint> {
    pub fn market(mut self, market: &str) -> Self {
        self.endpoint.market = Some(market.to_owned());
        self
    }

    pub async fn get(self) -> Result<Vec<SimplifiedShow>> {
        self.spotify
            .get("/shows/".to_owned(), self.endpoint)
            .await
            .map(|s: Shows| s.shows)
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct ShowEpisodesEndpoint {
    #[serde(skip)]
    pub(crate) show_id: String,
    pub(crate) market: Option<String>,
    pub(crate) limit: Option<Limit>,
    pub(crate) offset: Option<u32>,
}

impl<F: AuthFlow> Builder<'_, F, ShowEpisodesEndpoint> {
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

    pub async fn get(self) -> Result<Page<SimplifiedEpisode>> {
        self.spotify
            .get(
                format!("/shows/{}/episodes", self.endpoint.show_id),
                self.endpoint,
            )
            .await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct SavedShowsEndpoint {
    pub(crate) limit: Option<Limit>,
    pub(crate) offset: Option<u32>,
}

impl<F: AuthFlow> Builder<'_, F, SavedShowsEndpoint> {
    pub fn limit(mut self, limit: u32) -> Self {
        self.endpoint.limit = Some(Limit::new(limit));
        self
    }

    pub fn offset(mut self, offset: u32) -> Self {
        self.endpoint.offset = Some(offset);
        self
    }

    pub async fn get(self) -> Result<Page<SavedShow>> {
        self.spotify
            .get("/me/shows".to_owned(), self.endpoint)
            .await
    }
}

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
}
