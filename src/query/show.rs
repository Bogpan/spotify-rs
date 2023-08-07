use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct EpisodeQuery {
    #[serde(skip)]
    pub(crate) episode_id: String,
    market: Option<String>,
}

impl EpisodeQuery {
    pub fn new(episode_id: &str) -> Self {
        Self {
            episode_id: episode_id.to_owned(),
            market: None,
        }
    }

    pub fn market(mut self, market: &str) -> Self {
        self.market = Some(market.to_owned());
        self
    }
}

#[derive(Debug, Serialize)]
pub struct EpisodesQuery {
    #[serde(rename = "ids")]
    episode_ids: String,
    market: Option<String>,
}

impl EpisodesQuery {
    pub fn new(episode_ids: &[&str]) -> Self {
        Self {
            episode_ids: episode_ids.join(","),
            market: None,
        }
    }

    pub fn market(mut self, market: &str) -> Self {
        self.market = Some(market.to_owned());
        self
    }
}

#[derive(Debug, Default, Serialize)]
pub struct SavedEpisodesQuery {
    market: Option<String>,
    limit: Option<u32>,
    offset: Option<u32>,
}

impl SavedEpisodesQuery {
    pub fn new() -> Self {
        Self::default()
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
