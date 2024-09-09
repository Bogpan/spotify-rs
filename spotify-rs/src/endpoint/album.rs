use serde::Serialize;

use crate::{
    auth::{AuthFlow, Authorised},
    body_list,
    error::Result,
    model::{
        album::{Album, Albums, PagedAlbums, SavedAlbum, SimplifiedAlbum},
        track::SimplifiedTrack,
        Page,
    },
    query_list, Nil,
};

use super::{Client, Endpoint};

impl Endpoint for AlbumEndpoint {}
impl Endpoint for AlbumsEndpoint {}
impl Endpoint for AlbumTracksEndpoint {}
impl Endpoint for SavedAlbumsEndpoint {}
impl Endpoint for NewReleasesEndpoint {}

pub fn album(id: impl Into<String>) -> AlbumEndpoint {
    AlbumEndpoint {
        id: id.into(),
        market: None,
    }
}

pub fn albums<T: AsRef<str>>(ids: &[T]) -> AlbumsEndpoint {
    AlbumsEndpoint {
        ids: query_list(ids),
        market: None,
    }
}

pub fn album_tracks(album_id: impl Into<String>) -> AlbumTracksEndpoint {
    AlbumTracksEndpoint {
        id: album_id.into(),
        ..Default::default()
    }
}

// authorised only
pub fn saved_albums() -> SavedAlbumsEndpoint {
    SavedAlbumsEndpoint::default()
}

// authorised only
pub async fn save_albums<T: AsRef<str>>(
    ids: &[T],
    spotify: &Client<impl AuthFlow + Authorised>,
) -> Result<Nil> {
    spotify
        .put("/me/albums".to_owned(), body_list("ids", ids))
        .await
}

// authorised only
pub async fn remove_saved_albums<T: AsRef<str>>(
    ids: &[T],
    spotify: &Client<impl AuthFlow + Authorised>,
) -> Result<Nil> {
    spotify
        .delete("/me/albums".to_owned(), body_list("ids", ids))
        .await
}

// authorised only
pub async fn check_saved_albums<T: AsRef<str>>(
    ids: &[T],
    spotify: &Client<impl AuthFlow + Authorised>,
) -> Result<Vec<bool>> {
    spotify
        .get("/me/albums/contains".to_owned(), [("ids", query_list(ids))])
        .await
}

pub fn new_releases() -> NewReleasesEndpoint {
    NewReleasesEndpoint::default()
}

/// Endpoint for getting a single album.
#[derive(Clone, Debug, Default, Serialize)]
pub struct AlbumEndpoint {
    #[serde(skip)]
    pub(crate) id: String,
    pub(crate) market: Option<String>,
}

impl AlbumEndpoint {
    #[doc = include_str!("../docs/market.md")]
    pub fn market(mut self, market: impl Into<String>) -> Self {
        self.market = Some(market.into());
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn get(self, spotify: &Client<impl AuthFlow>) -> Result<Album> {
        spotify.get(format!("/albums/{}", self.id), self).await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct AlbumsEndpoint {
    pub(crate) ids: String,
    pub(crate) market: Option<String>,
}

impl AlbumsEndpoint {
    #[doc = include_str!("../docs/market.md")]
    pub fn market(mut self, market: impl Into<String>) -> Self {
        self.market = Some(market.into());
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn get(self, spotify: &Client<impl AuthFlow>) -> Result<Vec<Album>> {
        spotify
            .get("/albums".to_owned(), self)
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

impl AlbumTracksEndpoint {
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
    pub async fn get(self, spotify: &Client<impl AuthFlow>) -> Result<Page<SimplifiedTrack>> {
        spotify
            .get(format!("/albums/{}/tracks", self.id), self)
            .await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct SavedAlbumsEndpoint {
    pub(crate) market: Option<String>,
    pub(crate) limit: Option<u32>,
    pub(crate) offset: Option<u32>,
}

impl SavedAlbumsEndpoint {
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
    ) -> Result<Page<SavedAlbum>> {
        spotify.get("/me/albums".to_owned(), self).await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct NewReleasesEndpoint {
    pub(crate) country: Option<String>,
    pub(crate) limit: Option<u32>,
    pub(crate) offset: Option<u32>,
}

impl NewReleasesEndpoint {
    #[doc = include_str!("../docs/country.md")]
    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
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
    pub async fn get(self, spotify: &Client<impl AuthFlow>) -> Result<Page<SimplifiedAlbum>> {
        spotify
            .get("/browse/new-releases".to_owned(), self)
            .await
            .map(|p: PagedAlbums| p.albums)
    }
}
