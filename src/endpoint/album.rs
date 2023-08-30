use serde::Serialize;

use crate::{
    auth::AuthFlow,
    model::{
        album::{Album, Albums, PagedAlbums, SavedAlbum, SimplifiedAlbum},
        track::SimplifiedTrack,
        Page,
    },
    Result,
};

use super::{Builder, Endpoint, Limit};

impl Endpoint for AlbumEndpoint {}
impl Endpoint for AlbumsEndpoint {}
impl Endpoint for AlbumTracksEndpoint {}
impl Endpoint for SavedAlbumsEndpoint {}
impl Endpoint for NewReleasesEndpoint {}

#[derive(Clone, Debug, Default, Serialize)]
pub struct AlbumEndpoint {
    #[serde(skip)]
    pub(crate) id: String,
    pub(crate) market: Option<String>,
}

impl<F: AuthFlow> Builder<'_, F, AlbumEndpoint> {
    pub fn market(mut self, market: &str) -> Self {
        self.endpoint.market = Some(market.to_owned());
        self
    }

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

impl<F: AuthFlow> Builder<'_, F, AlbumsEndpoint> {
    pub fn market(mut self, market: &str) -> Self {
        self.endpoint.market = Some(market.to_owned());
        self
    }

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

impl<F: AuthFlow> Builder<'_, F, AlbumTracksEndpoint> {
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

impl<F: AuthFlow> Builder<'_, F, SavedAlbumsEndpoint> {
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

impl<F: AuthFlow> Builder<'_, F, NewReleasesEndpoint> {
    pub fn country(mut self, country: &str) -> Self {
        self.endpoint.country = Some(country.to_owned());
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

    pub async fn get(self) -> Result<Page<SimplifiedAlbum>> {
        self.spotify
            .get("/browse/new-releases".to_owned(), self.endpoint)
            .await
            .map(|p: PagedAlbums| p.albums)
    }
}
