use serde::Serialize;

use crate::{
    auth::{AuthFlow, Verifier},
    error::Result,
    model::{
        album::{AlbumGroup, SimplifiedAlbum},
        artist::{Artist, Artists},
        track::{Track, Tracks},
        Page,
    },
    query_list,
};

use super::{Builder, Endpoint, Limit};

impl Endpoint for ArtistAlbumsEndpoint {}
impl Endpoint for ArtistTopTracksEndpoint {}
impl Endpoint for ArtistEndpoint {}

#[derive(Clone, Debug, Default, Serialize)]
pub struct ArtistEndpoint {
    pub(crate) id: String,
}

impl<'a, F: AuthFlow, V: Verifier> Builder<'a, F, V, ArtistEndpoint> {
    pub fn albums(self) -> Builder<'a, F, V, ArtistAlbumsEndpoint> {
        Builder {
            spotify: self.spotify,
            endpoint: ArtistAlbumsEndpoint {
                id: self.endpoint.id,
                ..Default::default()
            },
        }
    }

    pub fn top_tracks(self) -> Builder<'a, F, V, ArtistTopTracksEndpoint> {
        Builder {
            spotify: self.spotify,
            endpoint: ArtistTopTracksEndpoint {
                id: self.endpoint.id,
                market: None,
            },
        }
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn get(self) -> Result<Artist> {
        self.spotify
            .get::<(), _>(format!("/artists/{}", self.endpoint.id), None)
            .await
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn get_related_artists(self) -> Result<Vec<Artist>> {
        self.spotify
            .get::<(), _>(
                format!("/artists/{}/related-artists", self.endpoint.id),
                None,
            )
            .await
            .map(|a: Artists| a.artists)
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct ArtistAlbumsEndpoint {
    #[serde(skip)]
    pub(crate) id: String,
    pub(crate) include_groups: Option<String>,
    pub(crate) market: Option<String>,
    pub(crate) limit: Option<Limit>,
    pub(crate) offset: Option<u32>,
}

impl<F: AuthFlow, V: Verifier> Builder<'_, F, V, ArtistAlbumsEndpoint> {
    /// Sets the album types to be returned. If not supplied all album types will be returned.
    pub fn include_groups(mut self, include_groups: &[AlbumGroup]) -> Self {
        self.endpoint.include_groups = Some(query_list(include_groups));
        self
    }

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
    pub async fn get(self) -> Result<Page<SimplifiedAlbum>> {
        self.spotify
            .get(
                format!("/artists/{}/albums", self.endpoint.id),
                self.endpoint,
            )
            .await
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct ArtistTopTracksEndpoint {
    #[serde(skip)]
    pub(crate) id: String,
    pub(crate) market: Option<String>,
}

impl<F: AuthFlow, V: Verifier> Builder<'_, F, V, ArtistTopTracksEndpoint> {
    #[doc = include_str!("../docs/market.md")]
    pub fn market(mut self, market: &str) -> Self {
        self.endpoint.market = Some(market.to_owned());
        self
    }

    #[doc = include_str!("../docs/send.md")]
    pub async fn get(self) -> Result<Vec<Track>> {
        self.spotify
            .get(
                format!("/artists/{}/top-tracks", self.endpoint.id),
                self.endpoint,
            )
            .await
            .map(|t: Tracks| t.tracks)
    }
}
