use serde::Serialize;

use crate::{
    auth::{AuthFlow, Verifier},
    error::Result,
    model::{
        show::{
            Episode, Episodes, SavedEpisode, SavedShow, Show, Shows, SimplifiedEpisode,
            SimplifiedShow,
        },
        Page,
    },
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

impl<F: AuthFlow, V: Verifier> Builder<'_, F, V, ShowEndpoint> {
    #[doc = include_str!("../docs/market.md")]
    pub fn market(mut self, market: impl Into<String>) -> Self {
        self.endpoint.market = Some(market.into());
        self
    }

    #[doc = include_str!("../docs/send.md")]
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

impl<F: AuthFlow, V: Verifier> Builder<'_, F, V, ShowsEndpoint> {
    #[doc = include_str!("../docs/market.md")]
    pub fn market(mut self, market: impl Into<String>) -> Self {
        self.endpoint.market = Some(market.into());
        self
    }

    // This doesn't flatten the result into a Vec<SimplifiedShow> because the user might want to
    // know that some of the shows they want return null.
    #[doc = include_str!("../docs/send.md")]
    pub async fn get(self) -> Result<Vec<Option<SimplifiedShow>>> {
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

impl<F: AuthFlow, V: Verifier> Builder<'_, F, V, ShowEpisodesEndpoint> {
    #[doc = include_str!("../docs/market.md")]
    pub fn market(mut self, market: impl Into<String>) -> Self {
        self.endpoint.market = Some(market.into());
        self
    }

    #[doc = include_str!("../docs/limit.md")]
    pub fn limit(mut self, limit: u32) -> Self {
        self.endpoint.limit = Some(Limit::new(limit));
        self
    }

    #[doc = include_str!("../docs/offset.md")]
    pub fn offset(mut self, offset: u32) -> Self {
        self.endpoint.offset = Some(offset);
        self
    }

    #[doc = include_str!("../docs/send.md")]
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

impl<F: AuthFlow, V: Verifier> Builder<'_, F, V, SavedShowsEndpoint> {
    #[doc = include_str!("../docs/limit.md")]
    pub fn limit(mut self, limit: u32) -> Self {
        self.endpoint.limit = Some(Limit::new(limit));
        self
    }

    #[doc = include_str!("../docs/offset.md")]
    pub fn offset(mut self, offset: u32) -> Self {
        self.endpoint.offset = Some(offset);
        self
    }

    #[doc = include_str!("../docs/send.md")]
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

impl<F: AuthFlow, V: Verifier> Builder<'_, F, V, EpisodeEndpoint> {
    #[doc = include_str!("../docs/market.md")]
    pub fn market(mut self, market: impl Into<String>) -> Self {
        self.endpoint.market = Some(market.into());
        self
    }

    #[doc = include_str!("../docs/send.md")]
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

impl<F: AuthFlow, V: Verifier> Builder<'_, F, V, EpisodesEndpoint> {
    #[doc = include_str!("../docs/market.md")]
    pub fn market(mut self, market: impl Into<String>) -> Self {
        self.endpoint.market = Some(market.into());
        self
    }

    #[doc = include_str!("../docs/send.md")]
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

impl<F: AuthFlow, V: Verifier> Builder<'_, F, V, SavedEpisodesEndpoint> {
    #[doc = include_str!("../docs/market.md")]
    pub fn market(mut self, market: impl Into<String>) -> Self {
        self.endpoint.market = Some(market.into());
        self
    }

    #[doc = include_str!("../docs/limit.md")]
    pub fn limit(mut self, limit: u32) -> Self {
        self.endpoint.limit = Some(Limit::new(limit));
        self
    }

    #[doc = include_str!("../docs/offset.md")]
    pub fn offset(mut self, offset: u32) -> Self {
        self.endpoint.offset = Some(offset);
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn get(self) -> Result<Page<SavedEpisode>> {
        self.spotify
            .get("/me/episodes".to_owned(), self.endpoint)
            .await
    }
}
