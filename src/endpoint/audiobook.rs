use serde::Serialize;

use crate::{
    auth::{AuthFlow, Verifier},
    error::Result,
    model::{
        audiobook::{
            Audiobook, Audiobooks, Chapter, Chapters, SimplifiedAudiobook, SimplifiedChapter,
        },
        Page,
    },
};

use super::{Builder, Endpoint, Limit};

impl Endpoint for AudiobookEndpoint {}
impl Endpoint for AudiobooksEndpoint {}
impl Endpoint for AudiobookChaptersEndpoint {}
impl Endpoint for SavedAudiobooksEndpoint {}
impl Endpoint for ChapterEndpoint {}
impl Endpoint for ChaptersEndpoint {}

#[derive(Clone, Debug, Default, Serialize)]
pub struct AudiobookEndpoint {
    #[serde(skip)]
    pub(crate) id: String,
    pub(crate) market: Option<String>,
}

impl<F: AuthFlow, V: Verifier> Builder<'_, F, V, AudiobookEndpoint> {
    #[doc = include_str!("../docs/market.md")]
    pub fn market(mut self, market: impl Into<String>) -> Self {
        self.endpoint.market = Some(market.into());
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn get(self) -> Result<Audiobook> {
        self.spotify
            .get(format!("/audiobooks/{}", self.endpoint.id), self.endpoint)
            .await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct AudiobooksEndpoint {
    pub(crate) ids: String,
    pub(crate) market: Option<String>,
}

impl<F: AuthFlow, V: Verifier> Builder<'_, F, V, AudiobooksEndpoint> {
    #[doc = include_str!("../docs/market.md")]
    pub fn market(mut self, market: impl Into<String>) -> Self {
        self.endpoint.market = Some(market.into());
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn get(self) -> Result<Vec<Audiobook>> {
        self.spotify
            .get("/audiobooks".to_owned(), self.endpoint)
            .await
            .map(|a: Audiobooks| a.audiobooks)
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct AudiobookChaptersEndpoint {
    #[serde(skip)]
    pub(crate) id: String,
    pub(crate) market: Option<String>,
    pub(crate) limit: Option<Limit>,
    pub(crate) offset: Option<u32>,
}

impl<F: AuthFlow, V: Verifier> Builder<'_, F, V, AudiobookChaptersEndpoint> {
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
    pub async fn get(self) -> Result<Page<SimplifiedChapter>> {
        self.spotify
            .get(
                format!("/audiobooks/{}/chapters", self.endpoint.id),
                self.endpoint,
            )
            .await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct SavedAudiobooksEndpoint {
    pub(crate) limit: Option<Limit>,
    pub(crate) offset: Option<u32>,
}

impl<F: AuthFlow, V: Verifier> Builder<'_, F, V, SavedAudiobooksEndpoint> {
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
    pub async fn get(self) -> Result<Page<SimplifiedAudiobook>> {
        // The map is required because the page's items might contain null (for some reason),
        // so this filters out the nulls.
        self.spotify
            .get("/me/audiobooks".to_owned(), self.endpoint)
            .await
            .map(|p: Page<Option<SimplifiedAudiobook>>| Page {
                href: p.href,
                limit: p.limit,
                next: p.next,
                offset: p.offset,
                previous: p.previous,
                total: p.total,
                items: p.items.into_iter().flatten().collect(),
            })
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct ChapterEndpoint {
    #[serde(skip)]
    pub(crate) id: String,
    pub(crate) market: Option<String>,
}

impl<F: AuthFlow, V: Verifier> Builder<'_, F, V, ChapterEndpoint> {
    #[doc = include_str!("../docs/market.md")]
    pub fn market(mut self, market: impl Into<String>) -> Self {
        self.endpoint.market = Some(market.into());
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn get(self) -> Result<Chapter> {
        self.spotify
            .get(format!("/chapters/{}", self.endpoint.id), self.endpoint)
            .await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct ChaptersEndpoint {
    pub(crate) ids: String,
    pub(crate) market: Option<String>,
}

impl<F: AuthFlow, V: Verifier> Builder<'_, F, V, ChaptersEndpoint> {
    #[doc = include_str!("../docs/market.md")]
    pub fn market(mut self, market: impl Into<String>) -> Self {
        self.endpoint.market = Some(market.into());
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn get(self) -> Result<Vec<Chapter>> {
        self.spotify
            .get("/chapters/".to_owned(), self.endpoint)
            .await
            .map(|c: Chapters| c.chapters)
    }
}
