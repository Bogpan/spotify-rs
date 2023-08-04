use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::*;

#[derive(Clone, Debug, Deserialize)]
pub struct Show {
    #[serde(default)]
    pub available_markets: Vec<String>,
    pub copyrights: Vec<String>,
    pub description: String,
    pub html_description: String,
    pub explicit: bool,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub is_externally_hosted: Option<bool>,
    pub languages: Vec<String>,
    pub media_type: String,
    pub name: String,
    pub publisher: String,
    pub r#type: String,
    pub uri: String,
    pub total_episodes: u32,
    pub episodes: Page<SimplifiedEpisode>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SimplifiedShow {
    #[serde(default)]
    pub available_markets: Vec<String>,
    pub copyrights: Vec<String>,
    pub description: String,
    pub html_description: String,
    pub explicit: bool,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub is_externally_hosted: Option<bool>,
    pub languages: Vec<String>,
    pub media_type: String,
    pub name: String,
    pub publisher: String,
    pub r#type: String,
    pub uri: String,
    pub total_episodes: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SavedShow {
    added_at: DateTime<Utc>,
    show: SimplifiedShow,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Episode {
    pub audio_preview_url: Option<String>,
    pub description: String,
    pub html_description: String,
    pub duration_ms: u32,
    pub explicit: bool,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub is_externally_hosted: bool,
    pub is_playable: bool,
    pub langauges: Vec<String>,
    pub name: String,
    pub release_date: String,
    pub release_date_precision: DatePrecision,
    pub resume_point: ResumePoint,
    pub r#type: String,
    pub uri: String,
    pub restrictions: Option<Restrictions>,
    pub show: SimplifiedShow,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SimplifiedEpisode {
    pub audio_preview_url: Option<String>,
    pub description: String,
    pub html_description: String,
    pub duration_ms: u32,
    pub explicit: bool,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub is_externally_hosted: bool,
    pub is_playable: bool,
    pub langauges: Vec<String>,
    pub name: String,
    pub release_date: String,
    pub release_date_precision: DatePrecision,
    pub resume_point: ResumePoint,
    pub r#type: String,
    pub uri: String,
    pub restrictions: Option<Restrictions>,
}
