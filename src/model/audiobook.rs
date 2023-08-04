use serde::Deserialize;

use super::*;

#[derive(Clone, Debug, Deserialize)]
pub struct Audiobook {
    pub authors: Vec<Author>,
    #[serde(default)]
    pub available_markets: Vec<String>,
    pub copyrights: Vec<Copyright>,
    pub description: String,
    pub html_description: String,
    pub edition: String,
    pub explicit: bool,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub languages: Vec<String>,
    pub media_type: String,
    pub name: String,
    pub narrators: Vec<Narrator>,
    pub publisher: String,
    pub r#type: String,
    pub uri: String,
    pub total_chapters: u32,
    pub chapters: Page<SimplifiedChapter>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SimplifiedAudiobook {
    pub authors: Vec<Author>,
    #[serde(default)]
    pub available_markets: Vec<String>,
    pub copyrights: Vec<Copyright>,
    pub description: String,
    pub html_description: String,
    pub edition: String,
    pub explicit: bool,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub languages: Vec<String>,
    pub media_type: String,
    pub name: String,
    pub narrators: Vec<Narrator>,
    pub publisher: String,
    pub r#type: String,
    pub uri: String,
    pub total_chapters: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Chapter {
    pub audio_preview_url: Option<String>,
    #[serde(default)]
    pub available_markets: Vec<String>,
    pub chapter_number: u32,
    pub description: String,
    pub html_description: String,
    pub duration_ms: u32,
    pub explicit: bool,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub is_playable: Option<bool>,
    pub languages: Vec<String>,
    pub name: String,
    pub release_date: String,
    pub release_date_precision: DatePrecision,
    pub resume_point: ResumePoint,
    pub r#type: String,
    pub uri: String,
    pub restrictions: Option<Restrictions>,
    pub audiobook: SimplifiedAudiobook,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SimplifiedChapter {
    pub audio_preview_url: Option<String>,
    #[serde(default)]
    pub available_markets: Vec<String>,
    pub chapter_number: u32,
    pub description: String,
    pub html_description: String,
    pub duration_ms: u32,
    pub explicit: bool,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub is_playable: Option<bool>,
    pub languages: Vec<String>,
    pub name: String,
    pub release_date: String,
    pub release_date_precision: DatePrecision,
    pub resume_point: ResumePoint,
    pub r#type: String,
    pub uri: String,
    pub restrictions: Option<Restrictions>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Author {
    pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Narrator {
    pub name: String,
}
