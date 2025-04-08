use serde::{Deserialize, Serialize};

pub mod album;
pub mod artist;
pub mod audio;
pub mod audiobook;
pub mod category;
pub mod market;
pub mod player;
pub mod playlist;
pub mod recommendation;
pub mod search;
pub mod show;
pub mod track;
pub mod user;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Page<T> {
    pub href: String,
    pub limit: u32,
    pub next: Option<String>,
    pub offset: u32,
    pub previous: Option<String>,
    pub total: u32,
    pub items: Vec<T>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CursorPage<T> {
    pub href: String,
    pub limit: u32,
    pub next: Option<String>,
    pub cursors: Cursor,
    pub total: Option<u32>,
    pub items: Vec<T>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Cursor {
    pub after: Option<String>,
    pub before: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Image {
    pub url: String,
    pub height: Option<u32>,
    pub width: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Copyright {
    pub text: String,
    pub r#type: CopyrightType,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Restrictions {
    pub reason: RestrictionReason,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExternalIds {
    pub isrc: Option<String>,
    pub ean: Option<String>,
    pub upc: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExternalUrls {
    pub spotify: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Followers {
    /// This will always be set to null, as the Web API does not support it at the moment.
    pub href: Option<String>,
    pub total: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ResumePoint {
    pub fully_played: bool,
    pub resume_position_ms: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RestrictionReason {
    Market,
    Product,
    Explicit,
    #[serde(other)]
    Unknown,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum CopyrightType {
    #[serde(rename = "C")]
    Copyright,
    #[serde(rename = "P")]
    Performance,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DatePrecision {
    Year,
    Month,
    Day,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PlayableItem {
    Track(track::Track),
    Episode(show::Episode),
}
