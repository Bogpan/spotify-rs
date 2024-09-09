use serde::Deserialize;

use super::track::Track;

/// Recommendations based on the available information for a given seed.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Recommendations {
    /// A list of seeds.
    pub seeds: Vec<RecommendationSeed>,
    /// A list of tracks, ordered according to the
    /// supplied parameters.
    pub tracks: Vec<Track>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RecommendationSeed {
    pub after_filtering_size: u32,
    pub after_relinking_size: u32,
    pub href: String,
    pub id: String,
    pub initial_pool_size: u32,
    pub r#type: String,
}

// Used only to deserialize JSON responses with arrays that are named objects.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub(crate) struct Genres {
    pub(crate) genres: Vec<String>,
}
