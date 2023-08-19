use serde::Serialize;

use crate::{
    auth::AuthFlow,
    model::{
        audiobook::{
            Audiobook, Audiobooks, Chapter, Chapters, SimplifiedAudiobook, SimplifiedChapter,
        },
        Page,
    },
    query_list, Nil, Result,
};

use super::{Builder, Endpoint};

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

impl<F: AuthFlow> Builder<'_, F, AudiobookEndpoint> {
    pub fn market(mut self, market: &str) -> Self {
        self.endpoint.market = Some(market.to_owned());
        self
    }

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

impl<F: AuthFlow> Builder<'_, F, AudiobooksEndpoint> {
    pub fn market(mut self, market: &str) -> Self {
        self.endpoint.market = Some(market.to_owned());
        self
    }

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
    pub(crate) limit: Option<u32>,
    pub(crate) offset: Option<u32>,
}

impl<F: AuthFlow> Builder<'_, F, AudiobookChaptersEndpoint> {
    pub fn market(mut self, market: &str) -> Self {
        self.endpoint.market = Some(market.to_owned());
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.endpoint.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: u32) -> Self {
        self.endpoint.offset = Some(offset);
        self
    }

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
    pub(crate) limit: Option<u32>,
    pub(crate) offset: Option<u32>,
}

impl<F: AuthFlow> Builder<'_, F, SavedAudiobooksEndpoint> {
    pub fn limit(mut self, limit: u32) -> Self {
        self.endpoint.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: u32) -> Self {
        self.endpoint.offset = Some(offset);
        self
    }

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

    pub async fn save<T: AsRef<str>>(self, ids: &[T]) -> Result<Nil> {
        self.spotify
            .put::<(), _>(format!("/me/audiobooks?ids={}", query_list(ids)), None)
            .await
    }

    pub async fn remove<T: AsRef<str>>(self, ids: &[T]) -> Result<Nil> {
        self.spotify
            .delete::<(), _>(format!("/me/audiobooks?ids={}", query_list(ids)), None)
            .await
    }

    pub async fn check<T: AsRef<str>>(self, ids: &[T]) -> Result<Vec<bool>> {
        self.spotify
            .get(
                "/me/audiobooks/contains".to_owned(),
                [("ids", query_list(ids))],
            )
            .await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct ChapterEndpoint {
    #[serde(skip)]
    pub(crate) id: String,
    pub(crate) market: Option<String>,
}

impl<F: AuthFlow> Builder<'_, F, ChapterEndpoint> {
    pub fn market(mut self, market: &str) -> Self {
        self.endpoint.market = Some(market.to_owned());
        self
    }

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

impl<F: AuthFlow> Builder<'_, F, ChaptersEndpoint> {
    pub fn market(mut self, market: &str) -> Self {
        self.endpoint.market = Some(market.to_owned());
        self
    }

    pub async fn get(self) -> Result<Vec<Chapter>> {
        self.spotify
            .get("/chapters/".to_owned(), self.endpoint)
            .await
            .map(|c: Chapters| c.chapters)
    }
}
