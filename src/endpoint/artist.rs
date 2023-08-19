use serde::Serialize;

use crate::{
    auth::AuthFlow,
    model::{
        album::{AlbumGroup, SimplifiedAlbum},
        artist::{Artist, Artists},
        track::{Track, Tracks},
        Page,
    },
    query_list, Result,
};

use super::{Builder, Endpoint};

impl Endpoint for ArtistAlbumsEndpoint {}
impl Endpoint for ArtistTopTracksEndpoint {}
impl Endpoint for ArtistEndpoint {}

#[derive(Clone, Debug, Default, Serialize)]
pub struct ArtistEndpoint {
    pub(crate) id: String,
}

impl<'a, F: AuthFlow> Builder<'a, F, ArtistEndpoint> {
    pub fn albums(self) -> Builder<'a, F, ArtistAlbumsEndpoint> {
        Builder {
            spotify: self.spotify,
            endpoint: ArtistAlbumsEndpoint {
                id: self.endpoint.id,
                ..Default::default()
            },
        }
    }

    pub fn top_tracks(self) -> Builder<'a, F, ArtistTopTracksEndpoint> {
        Builder {
            spotify: self.spotify,
            endpoint: ArtistTopTracksEndpoint {
                id: self.endpoint.id,
                market: None,
            },
        }
    }

    pub async fn get(self) -> Result<Artist> {
        self.spotify
            .get::<(), _>(format!("/artists/{}", self.endpoint.id), None)
            .await
    }

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
    pub(crate) limit: Option<u32>,
    pub(crate) offset: Option<u32>,
}

impl<F: AuthFlow> Builder<'_, F, ArtistAlbumsEndpoint> {
    pub fn include_groups(mut self, include_groups: &[AlbumGroup]) -> Self {
        self.endpoint.include_groups = Some(query_list(include_groups));
        self
    }

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

impl<F: AuthFlow> Builder<'_, F, ArtistTopTracksEndpoint> {
    pub fn market(mut self, market: &str) -> Self {
        self.endpoint.market = Some(market.to_owned());
        self
    }

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
