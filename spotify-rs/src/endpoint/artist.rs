use serde::Serialize;

use crate::{
    auth::AuthFlow,
    error::Result,
    model::{
        album::{AlbumGroup, SimplifiedAlbum},
        artist::{Artist, Artists},
        track::{Track, Tracks},
        Page,
    },
    query_list,
};

use super::{Client, Endpoint};

impl Endpoint for ArtistAlbumsEndpoint {}
impl Endpoint for ArtistTopTracksEndpoint {}

pub async fn get_artist(id: impl Into<String>, spotify: &Client<impl AuthFlow>) -> Result<Artist> {
    spotify
        .get::<(), _>(format!("/artists/{}", id.into()), None)
        .await
}

pub async fn get_artists<T: AsRef<str>>(
    ids: &[T],
    spotify: &Client<impl AuthFlow>,
) -> Result<Vec<Artist>> {
    spotify
        .get("/artists".to_owned(), [("ids", query_list(ids))])
        .await
        .map(|a: Artists| a.artists)
}

pub fn artist_albums(id: impl Into<String>) -> ArtistAlbumsEndpoint {
    ArtistAlbumsEndpoint {
        id: id.into(),
        ..Default::default()
    }
}

pub fn artist_top_tracks(id: impl Into<String>) -> ArtistTopTracksEndpoint {
    ArtistTopTracksEndpoint {
        id: id.into(),
        ..Default::default()
    }
}

/// **Note:** This endpoint has been deprecated by Spotify. It continues to work for
/// applications already using the extended mode in the API.
///
/// You can read more about this [here](https://developer.spotify.com/blog/2024-11-27-changes-to-the-web-api).
pub async fn get_related_artists(id: &str, spotify: &Client<impl AuthFlow>) -> Result<Vec<Artist>> {
    spotify
        .get::<(), _>(format!("/artists/{}/related-artists", id), None)
        .await
        .map(|a: Artists| a.artists)
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct ArtistAlbumsEndpoint {
    #[serde(skip)]
    pub(crate) id: String,
    pub(crate) include_groups: Option<String>,
    pub(crate) market: Option<String>,
    pub(crate) limit: Option<u32>,
    pub(crate) offset: Option<u32>,
}

impl ArtistAlbumsEndpoint {
    /// Sets the album types to be returned. If not supplied all album types will be returned.
    pub fn include_groups(mut self, include_groups: &[AlbumGroup]) -> Self {
        self.include_groups = Some(query_list(include_groups));
        self
    }

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
    pub async fn get(self, spotify: &Client<impl AuthFlow>) -> Result<Page<SimplifiedAlbum>> {
        spotify
            .get(format!("/artists/{}/albums", self.id), self)
            .await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct ArtistTopTracksEndpoint {
    #[serde(skip)]
    pub(crate) id: String,
    pub(crate) market: Option<String>,
}

impl ArtistTopTracksEndpoint {
    #[doc = include_str!("../docs/market.md")]
    pub fn market(mut self, market: impl Into<String>) -> Self {
        self.market = Some(market.into());
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn get(self, spotify: &Client<impl AuthFlow>) -> Result<Vec<Track>> {
        spotify
            .get(format!("/artists/{}/top-tracks", self.id), self)
            .await
            .map(|t: Tracks| t.tracks)
    }
}
