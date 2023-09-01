use serde::Serialize;

use crate::{
    auth::{AuthFlow, Verifier},
    error::Result,
    model::{
        album::{Album, Albums, PagedAlbums, SavedAlbum, SimplifiedAlbum},
        track::SimplifiedTrack,
        Page,
    },
};

use super::{Builder, Endpoint, Limit};

impl Endpoint for AlbumEndpoint {}
impl Endpoint for AlbumsEndpoint {}
impl Endpoint for AlbumTracksEndpoint {}
impl Endpoint for SavedAlbumsEndpoint {}
impl Endpoint for NewReleasesEndpoint {}

/// Endpoint for getting a single album.
#[derive(Clone, Debug, Default, Serialize)]
pub struct AlbumEndpoint {
    #[serde(skip)]
    pub(crate) id: String,
    pub(crate) market: Option<String>,
}

impl<F: AuthFlow, V: Verifier> Builder<'_, F, V, AlbumEndpoint> {
    #[doc = include_str!("../docs/market.md")]
    pub fn market(mut self, market: &str) -> Self {
        self.endpoint.market = Some(market.to_owned());
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn get(self) -> Result<Album> {
        self.spotify
            .get(format!("/albums/{}", self.endpoint.id), self.endpoint)
            .await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct AlbumsEndpoint {
    pub(crate) ids: String,
    pub(crate) market: Option<String>,
}

impl<F: AuthFlow, V: Verifier> Builder<'_, F, V, AlbumsEndpoint> {
    #[doc = include_str!("../docs/market.md")]
    pub fn market(mut self, market: &str) -> Self {
        self.endpoint.market = Some(market.to_owned());
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn get(self) -> Result<Vec<Album>> {
        self.spotify
            .get("/albums".to_owned(), self.endpoint)
            .await
            .map(|a: Albums| a.albums)
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct AlbumTracksEndpoint {
    #[serde(skip)]
    pub(crate) id: String,
    pub(crate) market: Option<String>,
    pub(crate) limit: Option<Limit>,
    pub(crate) offset: Option<u32>,
}

impl<F: AuthFlow, V: Verifier> Builder<'_, F, V, AlbumTracksEndpoint> {
    #[doc = include_str!("../docs/market.md")]
    pub fn market(mut self, market: &str) -> Self {
        self.endpoint.market = Some(market.to_owned());
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
    pub async fn get(self) -> Result<Page<SimplifiedTrack>> {
        self.spotify
            .get(
                format!("/albums/{}/tracks", self.endpoint.id),
                self.endpoint,
            )
            .await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct SavedAlbumsEndpoint {
    pub(crate) market: Option<String>,
    pub(crate) limit: Option<Limit>,
    pub(crate) offset: Option<u32>,
}

impl<F: AuthFlow, V: Verifier> Builder<'_, F, V, SavedAlbumsEndpoint> {
    #[doc = include_str!("../docs/market.md")]
    pub fn market(mut self, market: &str) -> Self {
        self.endpoint.market = Some(market.to_owned());
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
    pub async fn get(self) -> Result<Page<SavedAlbum>> {
        self.spotify
            .get("/me/albums".to_owned(), self.endpoint)
            .await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct NewReleasesEndpoint {
    pub(crate) country: Option<String>,
    pub(crate) limit: Option<Limit>,
    pub(crate) offset: Option<u32>,
}

impl<F: AuthFlow, V: Verifier> Builder<'_, F, V, NewReleasesEndpoint> {
    #[doc = include_str!("../docs/country.md")]
    pub fn country(mut self, country: &str) -> Self {
        self.endpoint.country = Some(country.to_owned());
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
    pub async fn get(self) -> Result<Page<SimplifiedAlbum>> {
        self.spotify
            .get("/browse/new-releases".to_owned(), self.endpoint)
            .await
            .map(|p: PagedAlbums| p.albums)
    }
}
