use serde::Serialize;

use crate::{
    auth::{AuthFlow, Authorised},
    body_list,
    error::Result,
    model::{
        show::{
            Episode, Episodes, SavedEpisode, SavedShow, Show, Shows, SimplifiedEpisode,
            SimplifiedShow,
        },
        Page,
    },
    query_list, Nil,
};

use super::{Client, Endpoint};

impl Endpoint for ShowEndpoint {}
impl Endpoint for ShowsEndpoint {}
impl Endpoint for ShowEpisodesEndpoint {}
impl Endpoint for SavedShowsEndpoint {}
impl Endpoint for EpisodeEndpoint {}
impl Endpoint for EpisodesEndpoint {}
impl Endpoint for SavedEpisodesEndpoint {}

pub fn show(id: impl Into<String>) -> ShowEndpoint {
    ShowEndpoint {
        id: id.into(),
        market: None,
    }
}

pub fn shows<T: AsRef<str>>(ids: &[T]) -> ShowsEndpoint {
    ShowsEndpoint {
        ids: query_list(ids),
        market: None,
    }
}

pub fn show_episodes(show_id: impl Into<String>) -> ShowEpisodesEndpoint {
    ShowEpisodesEndpoint {
        show_id: show_id.into(),
        ..Default::default()
    }
}

pub fn saved_shows() -> SavedShowsEndpoint {
    SavedShowsEndpoint::default()
}

pub async fn save_shows<T: AsRef<str>>(
    ids: &[T],
    spotify: &Client<impl AuthFlow + Authorised>,
) -> Result<Nil> {
    spotify
        .put("/me/shows".to_owned(), body_list("ids", ids))
        .await
}

pub async fn remove_saved_shows<T: AsRef<str>>(
    ids: &[T],
    spotify: &Client<impl AuthFlow + Authorised>,
) -> Result<Nil> {
    spotify
        .delete("/me/shows".to_owned(), body_list("ids", ids))
        .await
}

pub async fn check_saved_shows<T: AsRef<str>>(
    ids: &[T],
    spotify: &Client<impl AuthFlow + Authorised>,
) -> Result<Vec<bool>> {
    spotify
        .get("/me/shows/contains".to_owned(), [("ids", query_list(ids))])
        .await
}

pub fn episode(id: impl Into<String>) -> EpisodeEndpoint {
    EpisodeEndpoint {
        id: id.into(),
        market: None,
    }
}

pub fn episodes<T: AsRef<str>>(ids: &[T]) -> EpisodesEndpoint {
    EpisodesEndpoint {
        ids: query_list(ids),
        market: None,
    }
}

pub fn saved_episodes() -> SavedEpisodesEndpoint {
    SavedEpisodesEndpoint::default()
}

pub async fn save_episodes<T: AsRef<str>>(
    ids: &[T],
    spotify: &Client<impl AuthFlow + Authorised>,
) -> Result<Nil> {
    spotify
        .put("/me/episodes".to_owned(), body_list("ids", ids))
        .await
}

pub async fn remove_saved_episodes<T: AsRef<str>>(
    ids: &[T],
    spotify: &Client<impl AuthFlow + Authorised>,
) -> Result<Nil> {
    spotify
        .delete("/me/episodes".to_owned(), body_list("ids", ids))
        .await
}

pub async fn check_saved_episodes<T: AsRef<str>>(
    ids: &[T],
    spotify: &Client<impl AuthFlow + Authorised>,
) -> Result<Vec<bool>> {
    spotify
        .get::<(), _>(
            format!("/me/episodes/contains?ids={}", query_list(ids)),
            None,
        )
        .await
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct ShowEndpoint {
    #[serde(skip)]
    pub(crate) id: String,
    pub(crate) market: Option<String>,
}

impl ShowEndpoint {
    #[doc = include_str!("../docs/market.md")]
    pub fn market(mut self, market: impl Into<String>) -> Self {
        self.market = Some(market.into());
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn get(self, spotify: &Client<impl AuthFlow>) -> Result<Show> {
        spotify.get(format!("/shows/{}", self.id), self).await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct ShowsEndpoint {
    pub(crate) ids: String,
    pub(crate) market: Option<String>,
}

impl ShowsEndpoint {
    #[doc = include_str!("../docs/market.md")]
    pub fn market(mut self, market: impl Into<String>) -> Self {
        self.market = Some(market.into());
        self
    }

    // This doesn't flatten the result into a Vec<SimplifiedShow> because the user might want to
    // know that some of the shows they want return null.
    #[doc = include_str!("../docs/send.md")]
    pub async fn get(self, spotify: &Client<impl AuthFlow>) -> Result<Vec<Option<SimplifiedShow>>> {
        spotify
            .get("/shows/".to_owned(), self)
            .await
            .map(|s: Shows| s.shows)
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct ShowEpisodesEndpoint {
    #[serde(skip)]
    pub(crate) show_id: String,
    pub(crate) market: Option<String>,
    pub(crate) limit: Option<u32>,
    pub(crate) offset: Option<u32>,
}

impl ShowEpisodesEndpoint {
    #[doc = include_str!("../docs/market.md")]
    pub fn market(mut self, market: impl Into<String>) -> Self {
        self.market = Some(market.into());
        self
    }

    #[doc = include_str!("../docs/limit.md")]
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    #[doc = include_str!("../docs/offset.md")]
    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn get(self, spotify: &Client<impl AuthFlow>) -> Result<Page<SimplifiedEpisode>> {
        spotify
            .get(format!("/shows/{}/episodes", self.show_id), self)
            .await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct SavedShowsEndpoint {
    pub(crate) limit: Option<u32>,
    pub(crate) offset: Option<u32>,
}

impl SavedShowsEndpoint {
    #[doc = include_str!("../docs/limit.md")]
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    #[doc = include_str!("../docs/offset.md")]
    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn get(
        self,
        spotify: &Client<impl AuthFlow + Authorised>,
    ) -> Result<Page<SavedShow>> {
        spotify.get("/me/shows".to_owned(), self).await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct EpisodeEndpoint {
    #[serde(skip)]
    pub(crate) id: String,
    pub(crate) market: Option<String>,
}

impl EpisodeEndpoint {
    #[doc = include_str!("../docs/market.md")]
    pub fn market(mut self, market: impl Into<String>) -> Self {
        self.market = Some(market.into());
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn get(self, spotify: &Client<impl AuthFlow>) -> Result<Episode> {
        spotify.get(format!("/episodes/{}", self.id), self).await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct EpisodesEndpoint {
    pub(crate) ids: String,
    pub(crate) market: Option<String>,
}

impl EpisodesEndpoint {
    #[doc = include_str!("../docs/market.md")]
    pub fn market(mut self, market: impl Into<String>) -> Self {
        self.market = Some(market.into());
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn get(self, spotify: &Client<impl AuthFlow>) -> Result<Vec<Option<Episode>>> {
        spotify
            .get("/episodes/".to_owned(), self)
            .await
            .map(|e: Episodes| e.episodes)
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct SavedEpisodesEndpoint {
    pub(crate) market: Option<String>,
    pub(crate) limit: Option<u32>,
    pub(crate) offset: Option<u32>,
}

impl SavedEpisodesEndpoint {
    #[doc = include_str!("../docs/market.md")]
    pub fn market(mut self, market: impl Into<String>) -> Self {
        self.market = Some(market.into());
        self
    }

    #[doc = include_str!("../docs/limit.md")]
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    #[doc = include_str!("../docs/offset.md")]
    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn get(
        self,
        spotify: &Client<impl AuthFlow + Authorised>,
    ) -> Result<Page<SavedEpisode>> {
        spotify.get("/me/episodes".to_owned(), self).await
    }
}
