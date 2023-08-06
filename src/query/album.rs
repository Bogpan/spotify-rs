use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AlbumQuery {
    #[serde(skip)]
    pub(crate) album_id: String,
    market: Option<String>,
}

impl AlbumQuery {
    pub fn new(album_id: &str) -> Self {
        Self {
            album_id: album_id.to_owned(),
            market: None,
        }
    }

    pub fn market(mut self, market: &str) -> Self {
        self.market = Some(market.to_owned());
        self
    }
}

#[derive(Debug, Serialize)]
pub struct AlbumsQuery {
    #[serde(rename = "ids")]
    album_ids: String,
    market: Option<String>,
}

impl AlbumsQuery {
    pub fn new(album_ids: &[&str]) -> Self {
        Self {
            album_ids: album_ids.join(","),
            market: None,
        }
    }

    pub fn market(mut self, market: &str) -> Self {
        self.market = Some(market.to_owned());
        self
    }
}

#[derive(Debug, Serialize)]
pub struct AlbumTracksQuery {
    #[serde(skip)]
    pub(crate) album_id: String,
    market: Option<String>,
    limit: Option<u32>,
    offset: Option<u32>,
}

impl AlbumTracksQuery {
    pub fn new(album_id: &str) -> Self {
        Self {
            album_id: album_id.to_owned(),
            market: None,
            limit: None,
            offset: None,
        }
    }

    pub fn market(mut self, market: &str) -> Self {
        self.market = Some(market.to_owned());
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }
}

#[derive(Debug, Default, Serialize)]
pub struct SavedAlbumsQuery {
    limit: Option<u32>,
    offset: Option<u32>,
    market: Option<String>,
}

impl SavedAlbumsQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn market(mut self, market: &str) -> Self {
        self.market = Some(market.to_owned());
        self
    }
}

#[derive(Debug, Default, Serialize)]
pub struct NewReleaseQuery {
    limit: Option<u32>,
    offset: Option<u32>,
    country: Option<String>,
}

impl NewReleaseQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn country(mut self, country: &str) -> Self {
        self.country = Some(country.to_owned());
        self
    }
}
