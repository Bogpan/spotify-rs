use serde::Serialize;
use serde_json::json;

use crate::{
    auth::AuthFlow,
    client::Body,
    model::{
        album::{Album, Albums, PagedAlbums, SavedAlbum, SimplifiedAlbum},
        track::SimplifiedTrack,
        Page,
    },
    query_list, Nil, Result,
};

use super::{Builder, Endpoint};

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
    pub(crate) limit: Option<u32>,
    pub(crate) offset: Option<u32>,
}

impl<F: AuthFlow> Builder<'_, F, AlbumTracksEndpoint> {
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
    pub(crate) limit: Option<u32>,
    pub(crate) offset: Option<u32>,
}

impl<F: AuthFlow> Builder<'_, F, SavedAlbumsEndpoint> {
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

    pub async fn get(self) -> Result<Page<SavedAlbum>> {
        self.spotify
            .get("/me/albums".to_owned(), self.endpoint)
            .await
    }

    pub async fn save<T: Serialize>(self, ids: &[T]) -> Result<Nil> {
        self.spotify
            .put("/me/albums".to_owned(), Body::Json(json!({ "ids": ids })))
            .await
    }

    pub async fn remove<T: Serialize>(self, ids: &[T]) -> Result<Nil> {
        self.spotify
            .delete("/me/albums".to_owned(), Body::Json(json!({ "ids": ids })))
            .await
    }

    pub async fn check<T: AsRef<str>>(self, ids: &[T]) -> Result<Vec<bool>> {
        self.spotify
            .get("/me/albums/contains".to_owned(), [("ids", query_list(ids))])
            .await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct NewReleasesEndpoint {
    pub(crate) country: Option<String>,
    pub(crate) limit: Option<u32>,
    pub(crate) offset: Option<u32>,
}

impl<F: AuthFlow> Builder<'_, F, NewReleasesEndpoint> {
    pub fn country(mut self, country: &str) -> Self {
        self.endpoint.country = Some(country.to_owned());
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

    pub async fn get(self) -> Result<Page<SimplifiedAlbum>> {
        self.spotify
            .get("/browse/new-releases".to_owned(), self.endpoint)
            .await
            .map(|p: PagedAlbums| p.albums)
    }
}
