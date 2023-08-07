use serde::Deserialize;

use super::track::Track;

#[derive(Clone, Debug, Deserialize)]
pub struct Recommendations {
    pub seeds: Vec<RecommendationSeed>,
    pub tracks: Vec<Track>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RecommendationSeed {
    pub after_filtering_size: u32,
    pub after_relinking_size: u32,
    pub href: String,
    pub id: String,
    pub initial_pool_size: u32,
    pub r#type: String,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Genres {
    pub(crate) genres: Vec<String>,
}
