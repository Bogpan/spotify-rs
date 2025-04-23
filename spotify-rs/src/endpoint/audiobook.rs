use serde::Serialize;

use crate::{
    auth::{AuthFlow, Authorised},
    error::Result,
    model::{
        audiobook::{
            Audiobook, Audiobooks, Chapter, Chapters, SimplifiedAudiobook, SimplifiedChapter,
        },
        Page,
    },
    query_list, Nil,
};

use super::{Client, Endpoint};

impl Endpoint for AudiobookEndpoint {}
impl Endpoint for AudiobooksEndpoint {}
impl Endpoint for AudiobookChaptersEndpoint {}
impl Endpoint for SavedAudiobooksEndpoint {}
impl Endpoint for ChapterEndpoint {}
impl Endpoint for ChaptersEndpoint {}

#[doc = include_str!("../docs/client_creds_error.md")]
pub fn audiobook(id: impl Into<String>) -> AudiobookEndpoint {
    AudiobookEndpoint {
        id: id.into(),
        market: None,
    }
}

#[doc = include_str!("../docs/client_creds_error.md")]
pub fn audiobooks<T: AsRef<str>>(ids: &[T]) -> AudiobooksEndpoint {
    AudiobooksEndpoint {
        ids: query_list(ids),
        market: None,
    }
}

#[doc = include_str!("../docs/client_creds_error.md")]
pub fn audiobook_chapters(audiobook_id: impl Into<String>) -> AudiobookChaptersEndpoint {
    AudiobookChaptersEndpoint {
        id: audiobook_id.into(),
        ..Default::default()
    }
}

pub fn saved_audiobooks() -> SavedAudiobooksEndpoint {
    SavedAudiobooksEndpoint::default()
}

pub async fn save_audiobooks<T: AsRef<str>>(
    ids: &[T],
    spotify: &Client<impl AuthFlow + Authorised>,
) -> Result<Nil> {
    spotify
        .put::<(), _>(format!("/me/audiobooks?ids={}", query_list(ids)), None)
        .await
}

pub async fn remove_saved_audiobooks<T: AsRef<str>>(
    ids: &[T],
    spotify: &Client<impl AuthFlow + Authorised>,
) -> Result<Nil> {
    spotify
        .delete::<(), _>(format!("/me/audiobooks?ids={}", query_list(ids)), None)
        .await
}

pub async fn check_saved_audiobooks<T: AsRef<str>>(
    ids: &[T],
    spotify: &Client<impl AuthFlow + Authorised>,
) -> Result<Vec<bool>> {
    spotify
        .get(
            "/me/audiobooks/contains".to_owned(),
            [("ids", query_list(ids))],
        )
        .await
}

#[doc = include_str!("../docs/client_creds_error.md")]
pub fn chapter(id: impl Into<String>) -> ChapterEndpoint {
    ChapterEndpoint {
        id: id.into(),
        market: None,
    }
}

#[doc = include_str!("../docs/client_creds_error.md")]
pub fn chapters<T: AsRef<str>>(ids: &[T]) -> ChaptersEndpoint {
    ChaptersEndpoint {
        ids: query_list(ids),
        market: None,
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct AudiobookEndpoint {
    #[serde(skip)]
    pub(crate) id: String,
    pub(crate) market: Option<String>,
}

impl AudiobookEndpoint {
    #[doc = include_str!("../docs/market.md")]
    pub fn market(mut self, market: impl Into<String>) -> Self {
        self.market = Some(market.into());
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn get(self, spotify: &Client<impl AuthFlow>) -> Result<Audiobook> {
        spotify.get(format!("/audiobooks/{}", self.id), self).await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct AudiobooksEndpoint {
    pub(crate) ids: String,
    pub(crate) market: Option<String>,
}

impl AudiobooksEndpoint {
    #[doc = include_str!("../docs/market.md")]
    pub fn market(mut self, market: impl Into<String>) -> Self {
        self.market = Some(market.into());
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn get(self, spotify: &Client<impl AuthFlow>) -> Result<Vec<Option<Audiobook>>> {
        spotify
            .get("/audiobooks".to_owned(), self)
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

impl AudiobookChaptersEndpoint {
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
    pub async fn get(self, spotify: &Client<impl AuthFlow>) -> Result<Page<SimplifiedChapter>> {
        spotify
            .get(format!("/audiobooks/{}/chapters", self.id), self)
            .await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct SavedAudiobooksEndpoint {
    pub(crate) limit: Option<u32>,
    pub(crate) offset: Option<u32>,
}

impl SavedAudiobooksEndpoint {
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
    ) -> Result<Page<SimplifiedAudiobook>> {
        // The map is required because the page's items might contain null (for some reason),
        // so this filters out the nulls.
        spotify.get("/me/audiobooks".to_owned(), self).await.map(
            |p: Page<Option<SimplifiedAudiobook>>| Page {
                href: p.href,
                limit: p.limit,
                next: p.next,
                offset: p.offset,
                previous: p.previous,
                total: p.total,
                items: p.items.into_iter().flatten().collect(),
            },
        )
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct ChapterEndpoint {
    #[serde(skip)]
    pub(crate) id: String,
    pub(crate) market: Option<String>,
}

impl ChapterEndpoint {
    #[doc = include_str!("../docs/market.md")]
    pub fn market(mut self, market: impl Into<String>) -> Self {
        self.market = Some(market.into());
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn get(self, spotify: &Client<impl AuthFlow>) -> Result<Chapter> {
        spotify.get(format!("/chapters/{}", self.id), self).await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct ChaptersEndpoint {
    pub(crate) ids: String,
    pub(crate) market: Option<String>,
}

impl ChaptersEndpoint {
    #[doc = include_str!("../docs/market.md")]
    pub fn market(mut self, market: impl Into<String>) -> Self {
        self.market = Some(market.into());
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn get(self, spotify: &Client<impl AuthFlow>) -> Result<Vec<Option<Chapter>>> {
        spotify
            .get("/chapters/".to_owned(), self)
            .await
            .map(|c: Chapters| c.chapters)
    }
}
