use serde::Serialize;

use crate::model::album::AlbumGroup;

#[derive(Debug, Serialize)]
pub struct ArtistAlbumsQuery {
    #[serde(skip)]
    pub(crate) artist_id: String,
    // include_groups: Option<Vec<AlbumGroup>>,
    include_groups: Option<String>,
    market: Option<String>,
    limit: Option<u32>,
}

impl ArtistAlbumsQuery {
    pub fn new(artist_id: &str) -> Self {
        Self {
            artist_id: artist_id.to_owned(),
            include_groups: None,
            market: None,
            limit: None,
        }
    }

    pub fn include_groups(mut self, include_groups: &[AlbumGroup]) -> Self {
        self.include_groups = Some(
            include_groups
                .iter()
                .map(|g| g.to_string())
                .collect::<Vec<String>>()
                .join(","),
        );
        self
    }

    pub fn market(mut self, market: &str) -> Self {
        self.market = Some(market.to_owned());
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

#[derive(Debug, Serialize)]
pub struct ArtistTopTracksQuery {
    #[serde(skip)]
    pub(crate) artist_id: String,
    market: Option<String>,
}

impl ArtistTopTracksQuery {
    pub fn new(artist_id: &str) -> Self {
        Self {
            artist_id: artist_id.to_owned(),
            market: None,
        }
    }

    pub fn market(mut self, market: &str) -> Self {
        self.market = Some(market.to_owned());
        self
    }
}
