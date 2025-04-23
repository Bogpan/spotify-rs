use chrono::{DateTime, Utc};
use serde::Deserialize;
use spotify_rs_macros::docs;

use super::*;

/// A show.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[docs]
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
    /// Whether or not all of the show's episodes are hosted outside of Spotify's
    /// CDN.
    pub is_externally_hosted: Option<bool>,
    pub languages: Vec<String>,
    pub media_type: String,
    pub name: String,
    pub publisher: String,
    pub r#type: String,
    pub uri: String,
    /// The amount of episodes the show contains.
    pub total_episodes: u32,
    /// The episodes of the show.
    pub episodes: Page<SimplifiedEpisode>,
}

/// A simplified show, missing some details, that is usually obtained
/// through endpoints not specific to shows. The `href` field may be
/// used to get a full show.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[docs(name = "show")]
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
    /// Whether or not all of the show's episodes are hosted outside of Spotify's
    /// CDN.
    pub is_externally_hosted: Option<bool>,
    pub languages: Vec<String>,
    pub media_type: String,
    pub name: String,
    pub publisher: String,
    pub r#type: String,
    pub uri: String,
    /// The amount of episodes the show contains.
    pub total_episodes: u32,
}

/// A show saved by a user.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct SavedShow {
    /// The date and time the show was saved.
    pub added_at: DateTime<Utc>,
    /// The show itself.
    pub show: SimplifiedShow,
}

// Used only to deserialize JSON responses with arrays that are named objects.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub(crate) struct Shows {
    pub(crate) shows: Vec<Option<SimplifiedShow>>,
}

/// A show episode.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[docs]
pub struct Episode {
    /// The URL for a 30 second MP3 preview of the chapter.
    ///
    /// **Note:** This attribute has been deprecated by Spotify. It continues to work for
    /// applications already using the extended mode in the API.
    ///
    /// You can read more about this [here](https://developer.spotify.com/blog/2024-11-27-changes-to-the-web-api).
    pub audio_preview_url: Option<String>,
    pub description: String,
    pub html_description: String,
    pub duration_ms: u32,
    pub explicit: bool,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    /// Whether or not the episode is hosted outside of Spotify's CDN.
    pub is_externally_hosted: bool,
    pub is_playable: bool,
    pub languages: Vec<String>,
    pub name: String,
    pub release_date: String,
    pub release_date_precision: DatePrecision,
    pub resume_point: Option<ResumePoint>,
    pub r#type: String,
    pub uri: String,
    /// Included in the response when a content restriction is applied.
    pub restrictions: Option<Restriction>,
    /// The show to which the episode belongs.
    pub show: SimplifiedShow,
}

/// A simplified episode, missing some details, that is usually obtained
/// through endpoints not specific to episodes. The `href` field may be
/// used to get a full episode.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[docs(name = "episode")]
pub struct SimplifiedEpisode {
    /// The URL for a 30 second MP3 preview of the chapter.
    ///
    /// **Note:** This attribute has been deprecated by Spotify. It continues to work for
    /// applications already using the extended mode in the API.
    ///
    /// You can read more about this [here](https://developer.spotify.com/blog/2024-11-27-changes-to-the-web-api).
    pub audio_preview_url: Option<String>,
    pub description: String,
    pub html_description: String,
    pub duration_ms: u32,
    pub explicit: bool,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    /// Whether or not the episode is hosted outside of Spotify's CDN.
    pub is_externally_hosted: bool,
    pub is_playable: bool,
    pub languages: Vec<String>,
    pub name: String,
    pub release_date: String,
    pub release_date_precision: DatePrecision,
    pub resume_point: Option<ResumePoint>,
    pub r#type: String,
    pub uri: String,
    /// Included in the response when a content restriction is applied.
    pub restrictions: Option<Restriction>,
}

/// An episode saved by a user.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct SavedEpisode {
    /// The date and time the episode was saved.
    pub added_at: DateTime<Utc>,
    /// The episode itself.
    pub episode: Episode,
}

// Used only to deserialize JSON responses with arrays that are named objects.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub(crate) struct Episodes {
    pub(crate) episodes: Vec<Option<Episode>>,
}
