use serde::{Deserialize, Serialize};

use super::track::Track;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Recommendations {
    pub seeds: Vec<RecommendationSeed>,
    pub tracks: Vec<Track>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RecommendationSeed {
    pub after_filtering_size: u32,
    pub after_relinking_size: u32,
    pub href: String,
    pub id: String,
    pub initial_pool_size: u32,
    pub r#type: String,
}
